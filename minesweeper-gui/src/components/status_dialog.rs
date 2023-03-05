use relm4::{
  gtk, gtk::prelude::*, ComponentParts, ComponentSender, SimpleComponent,
};

pub struct StatusDialogModel {
  hidden: bool,
  message: Option<String>,
}

#[derive(Debug)]
pub enum StatusMsg {
  Open(String),
  Close,
}

#[relm4::component(pub)]
impl SimpleComponent for StatusDialogModel {
  type Input = StatusMsg;
  type Output = ();
  type Init = bool;

  view! {
    gtk::MessageDialog {
      set_modal: true,
      #[watch]
      set_visible: !model.hidden,
      #[watch]
      set_text: model.message.as_deref(),
      set_css_classes: &["status_dialog"],
      set_decorated: true,
      set_message_type: gtk::MessageType::Info,
      add_button: ("Close", gtk::ResponseType::Close),
      connect_response[sender] => move |_, _resp| {
        sender.input(StatusMsg::Close)
      }
    }
  }

  fn init(
    hidden: Self::Init,
    root: &Self::Root,
    sender: ComponentSender<Self>,
  ) -> relm4::ComponentParts<Self> {
    let model = StatusDialogModel {
      hidden,
      message: None,
    };

    let widgets = view_output!();
    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    match msg {
      StatusMsg::Open(message) => {
        self.message = Some(message);
        self.hidden = false;
      }
      StatusMsg::Close => self.hidden = true,
    }
  }
}
