{-# LANGUAGE TemplateHaskell, FlexibleContexts #-}

module MineSweeper
  ( Board
  , Cell(..)
  , CellState(..)
  , GameEnv(..)
  , Pos
    -- lenses / prisms
  , pos
  , coveredLens
  , coveredFlaggedLens
  , coveredMinedLens
  , xCoordLens
  , yCoordLens
    -- Functions
  , emptyBoard
  , groupedByRows
  , displayCell
  , isLoss
  , isWin
  , exposeMines
  , openCell
  , flagCell
  , mineBoard
  )
where

import Control.Lens  ((%~), (&), (.~), (^.), (^?), Lens', Traversal', _1, _2,
                      anyOf, filtered, folded, lengthOf, makeLenses, makePrisms,
                      preview, to, view)
import Control.Monad.Reader (ask, asks, MonadReader, MonadIO)
import Data.Function (on)
import Data.List     (find, groupBy, nub, delete, sortBy)
import Data.Maybe    (isJust)
import System.Random (getStdGen, getStdRandom, randomR, randomRs)

type Pos = (Int, Int)
type Board = [Cell]

data CellState = Covered   { _mined :: Bool, _flagged :: Bool }
               | UnCovered { _mined :: Bool }
               deriving (Show, Eq)

data Cell = Cell
          { _pos :: Pos
          , _state :: CellState
          , _cellId :: Int
          , _adjacentMines :: Int }
          deriving (Show, Eq)

data GameEnv = GameEnv { rows    :: Int
                       , columns :: Int }

makePrisms ''CellState
makeLenses ''CellState
makeLenses ''Cell

-- Re-useable lens.
coveredLens :: Traversal' Cell (Bool, Bool)
coveredLens = state . _Covered

coveredMinedLens, coveredFlaggedLens, unCoveredLens :: Traversal' Cell Bool
coveredMinedLens = coveredLens . _1
coveredFlaggedLens = coveredLens . _2
unCoveredLens = state . _UnCovered

xCoordLens, yCoordLens :: Lens' Cell Int
xCoordLens = pos . _1
yCoordLens = pos . _2

emptyBoard :: MonadReader GameEnv m => m Board
emptyBoard = do
    gs <- ask
    let positions = (,) <$> [1..columns gs] <*> [1..rows gs]
    pure $ (\(n, p) -> Cell { _pos = p
                            , _state = Covered False False
                            , _adjacentMines = 0
                            , _cellId = n }) <$> zip [1..] positions

updateCell :: Cell -> Board -> Board
updateCell cell = fmap (\c -> if cell ^. cellId == c ^. cellId then cell else c)

updateBoard :: Board -> [Cell] -> Board
updateBoard = foldr updateCell

okToOpen :: [Cell] -> [Cell]
okToOpen = filter (\c -> c ^? coveredLens == Just (False, False))

openUnMined :: Cell -> Cell
openUnMined = state .~ UnCovered False

openCell :: MonadReader GameEnv m => Pos -> Board -> m Board
openCell p b = f $ find (\c -> c ^. pos == p) b
 where
  f (Just c) | c ^? coveredFlaggedLens == Just True = pure b
             | c ^? coveredMinedLens == Just True   = pure $ updateCell
               (c & state .~ UnCovered True) b
             | isCovered c = isFirstMove b >>= \b' ->
                if c ^. adjacentMines == 0 && not b'
                then pure . updateCell (openUnMined c) $ expandEmptyCells b c
                else pure $ updateCell (openUnMined c) b
             | otherwise = pure b
  f Nothing = pure b
  isCovered = isJust . preview coveredLens

expandEmptyCells :: Board -> Cell -> Board
expandEmptyCells board cell
  | null openedCells = board
  | otherwise = foldr (flip expandEmptyCells) updatedBoard (zeroAdjacent openedCells)
 where
  findMore _ [] = []
  findMore exclude (c : xs)
    | c `elem` exclude        = findMore exclude xs
    | c ^. adjacentMines == 0 = c : adjacent c <>
      findMore (c : exclude <> adjacent c) xs
    | otherwise               = c : findMore (c : exclude) xs
  adjacent     = okToOpen . flip adjacentCells board
  openedCells  = openUnMined <$> nub (findMore [cell] (adjacent cell))
  zeroAdjacent = filter (view (adjacentMines . to (== 0)))
  updatedBoard = updateBoard board openedCells

flagCell :: Pos -> Board -> Board
flagCell p board = case find ((== p) . view pos) board of
  Just c  -> updateCell (c & state . flagged %~ not) board
  Nothing -> board

adjacentCells :: Cell -> Board -> [Cell]
adjacentCells Cell {_pos = c@(x1, y1)} = filter (\c -> c ^. pos `elem` positions)
  where
    f n = [pred n, n, succ n]
    positions = delete c $ [(x, y) | x <- f x1, x > 0, y <- f y1, y > 0]

isLoss, isWin, allUnMinedOpen, allMinesFlagged :: Board -> Bool
isLoss = anyOf (traverse . unCoveredLens) (== True)
isWin b = allUnMinedOpen b || allMinesFlagged b

allUnMinedOpen = (== 0) . lengthOf (traverse . coveredMinedLens . filtered (== False))
allMinesFlagged b = minedCount b == flaggedMineCount b
 where
  minedCount = lengthOf (traverse . coveredMinedLens . filtered (== True))
  flaggedMineCount = lengthOf (traverse . coveredLens . filtered (== (True, True)))

isFirstMove :: MonadReader GameEnv m => Board -> m Bool
isFirstMove b = asks totalCells >>= \n ->
  pure . (== n) $ lengthOf (folded . coveredFlaggedLens . filtered (== False)) b

groupedByRows :: Board -> [Board]
groupedByRows = let yAxis = view yCoordLens
                in groupBy ((==) `on` yAxis) . sortBy (compare `on` yAxis)

displayCell :: Cell -> String
displayCell c
  | c ^? unCoveredLens            == Just True = "X"
  | c ^? coveredFlaggedLens       == Just True = "?"
  | c ^? (unCoveredLens . to not) == Just True =
    if c ^. adjacentMines > 0 then show $ c ^. adjacentMines else "▢"
  | otherwise = "."

exposeMines :: Board -> Board
exposeMines = fmap (\c -> c & state . filtered (\s -> s ^? _Covered . _1 == Just True) .~ UnCovered True)

updateMineCount :: Board -> Board
updateMineCount b = go b
 where
  go []       = []
  go (x : xs) = (x & adjacentMines .~ totalAdjacentMines b) : go xs
   where
    totalAdjacentMines =
      foldr (\c acc -> if c ^. (state . mined) then succ acc else acc) 0 . adjacentCells x

mineBoard :: (MonadReader GameEnv m, MonadIO m) => Pos -> Board -> m Board
mineBoard p board = do
  totalMines <- randomMinedCount
  b <- minedBoard totalMines
  pure $ updateMineCount b
 where
  mines n = take n <$> randomCellIds
  minedBoard n = (\m ->
        fmap (\c -> if c ^. cellId `elem` m
                    then c & state . mined .~ True
                    else c)
        board) . filter (\c -> openedCell ^. cellId /= c)
        <$> mines n
  openedCell = head $ filter (\c -> c ^. pos == p) board

totalCells :: GameEnv -> Int
totalCells = (*) <$> rows <*> columns

randomCellIds :: (MonadReader GameEnv m, MonadIO m) => m [Int]
randomCellIds = asks totalCells >>= \n -> randomRs (1, n) <$> getStdGen

randomMinedCount :: (MonadReader GameEnv m, MonadIO m) => m Int
randomMinedCount = do
    n <- asks totalCells
    let maxMinedCells = floor $ realToFrac n * 0.2
        minMinedCells = floor $ realToFrac n * 0.1
    getStdRandom $ randomR (minMinedCells, maxMinedCells)
