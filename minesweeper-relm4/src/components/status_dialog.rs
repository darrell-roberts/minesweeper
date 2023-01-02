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
    gtk::Window {
      set_modal: true,
      #[watch]
      set_visible: !model.hidden,
      set_default_width: 250,
      set_default_height: 200,
      set_decorated: false,
      set_css_classes: &["status_dialog"],

      #[wrap(Some)]
      set_child = &gtk::Box {
          set_orientation: gtk::Orientation::Vertical,
          gtk::Label {
              #[watch]
              set_label: model.message.as_deref().unwrap_or_default(),
              set_css_classes: &["statusMessage"],
          },
          gtk::Box {
              set_orientation: gtk::Orientation::Vertical,
              set_vexpand: true,

          },
          gtk::Button {
             set_label: "Close",
             connect_clicked => StatusMsg::Close
          },
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
