use crate::AppModel;
use relm4::{gtk::prelude::*, send, ComponentUpdate, Model, Widgets};

pub struct StatusDialogModel {
  hidden: bool,
  message: Option<String>,
}

impl Model for StatusDialogModel {
  type Components = ();
  type Msg = StatusMsg;
  type Widgets = StatusDialogWidgets;
}

pub enum StatusMsg {
  Open(String),
  Close,
}

impl ComponentUpdate<AppModel> for StatusDialogModel {
  fn init_model(_parent_model: &AppModel) -> Self {
    Self {
      hidden: true,
      message: None,
    }
  }

  fn update(
    &mut self,
    msg: Self::Msg,
    _components: &Self::Components,
    _sender: relm4::Sender<Self::Msg>,
    _parent_sender: relm4::Sender<<AppModel as relm4::Model>::Msg>,
  ) {
    match msg {
      StatusMsg::Open(message) => {
        self.message = Some(message);
        self.hidden = false;
      }
      StatusMsg::Close => self.hidden = true,
    }
  }
}

pub struct StatusDialogWidgets {
  dialog: gtk::MessageDialog,
}

impl Widgets<StatusDialogModel, AppModel> for StatusDialogWidgets {
  type Root = gtk::MessageDialog;

  fn init_view(
    _model: &StatusDialogModel,
    _components: &<StatusDialogModel as Model>::Components,
    sender: relm4::Sender<<StatusDialogModel as Model>::Msg>,
  ) -> Self {
    let dialog = gtk::MessageDialog::builder()
      .modal(true)
      .visible(false)
      .message_type(gtk::MessageType::Info)
      .css_name("status_dialog")
      .decorated(true)
      // .title("Game Over")
      .build();

    dialog.add_button("Close", gtk::ResponseType::Close);
    dialog.connect_response(move |_, _| send!(sender, StatusMsg::Close));

    Self { dialog }
  }

  fn connect_parent(&mut self, parent_widgets: &<AppModel as Model>::Widgets) {
    self
      .dialog
      .set_transient_for(Some(&parent_widgets.root_widget()))
  }

  fn root_widget(&self) -> Self::Root {
    self.dialog.clone()
  }

  fn view(
    &mut self,
    model: &StatusDialogModel,
    _sender: relm4::Sender<<StatusDialogModel as Model>::Msg>,
  ) {
    self.dialog.set_visible(!model.hidden);
    self.dialog.set_text(model.message.as_deref());
  }
}
