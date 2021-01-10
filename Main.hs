{-# LANGUAGE FlexibleContexts #-}

module Main where

import Control.Lens         (lengthOf, filtered, view, preview)
import Control.Monad        (guard, (>=>))
import Control.Monad.Reader (asks, runReaderT, liftIO, MonadReader, MonadIO)
import MineSweeper          (Board, Cell(..), Pos, coveredLens, coveredMinedLens,
                             coveredFlaggedLens, xCoordLens, yCoordLens, pos, emptyBoard,
                             groupedByRows, displayCell, isLoss, isWin, exposeMines,
                             openCell, flagCell, mineBoard, GameEnv(..))
import System.IO            (hSetBuffering, stdout, BufferMode(..))
import System.Environment   (getArgs)
import Text.Printf          (printf)
import Text.Read            (readMaybe)

data Command = Open Pos | Flag Pos | Invalid

parseInput :: MonadReader GameEnv m => String -> m Command
parseInput s | length input /= 3 = pure Invalid
             | otherwise         = asks (maybe Invalid command . parsedPos)
 where
  input     = words s
  parsedPos (GameEnv r c) = do
    x <- readMaybe (input !! 1)
    y <- readMaybe (input !! 2)
    guard (x <= c && y <= r)
    pure (x, y)
  command p = case head input of "o" -> Open p
                                 "f" -> Flag p
                                 _   -> Invalid

cheat :: Board -> String
cheat = show . fmap (view pos) . filter ((== Just True) . preview coveredMinedLens)

totalMines, totalFlagged, totalCovered :: Board -> Int
totalMines = lengthOf (traverse . coveredMinedLens . filtered (==True))
totalFlagged = lengthOf (traverse . coveredFlaggedLens . filtered (==True))
totalCovered = lengthOf (traverse . coveredLens)

-- IO
drawBoard :: Board -> IO ()
drawBoard b = do
  -- printf "Cheat: %s\n" $ cheat b
  printf "  Mines: %d Covered: %d Flagged: %d\n\n"
    (totalMines b) (totalCovered b) (totalFlagged b)
  printf "%3s" ""
  mapM_ (printf "%3d") $ view xCoordLens <$> head rows
  printf "\n"
  mapM_ (\row -> do
    printf "%3d" $ yCoord row
    mapM_ (printf "%3s" . displayCell) row
    printf "\n" ) rows
 where
  rows   = groupedByRows b
  yCoord = view yCoordLens . head

gameLoop :: (MonadReader GameEnv m, MonadIO m) => Board -> m ()
gameLoop b
  | isLoss b  = liftIO $ drawBoard (exposeMines b) >> printf "\nYou Lose.\n"
  | isWin b   = liftIO $ drawBoard b >> printf "\nYou Win.\n"
  | otherwise = do
              line <- liftIO $ promptUser b
              command <- parseInput line
              case command of Open p  -> openCell p b >>= gameLoop
                              Flag p  -> gameLoop $ flagCell p b
                              Invalid -> gameLoop b

parseArgs :: [String] -> Maybe GameEnv
parseArgs (x:y:xs) = GameEnv <$> readMaybe x <*> readMaybe y
parseArgs _        = Nothing

promptUser :: Board -> IO String
promptUser = drawBoard >=> \_ -> putStr "\nPick a cell: " >> getLine

startGame :: (MonadReader GameEnv m, MonadIO m) => m ()
startGame = do
  board <- emptyBoard
  line <- liftIO $ promptUser board
  command <- parseInput line
  case command of Open p -> mineBoard p board >>= openCell p >>= gameLoop
                  _      -> startGame

main :: IO ()
main = do
  args <- hSetBuffering stdout NoBuffering >> getArgs
  case parseArgs args of Just env -> runReaderT startGame env
                         Nothing  -> runReaderT startGame $ GameEnv 20 20
