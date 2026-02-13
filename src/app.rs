use cosmic::iced::window::Id;
use cosmic::iced::{Limits, Subscription};
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::widget;
use std::time::Duration;

use crate::config::VoiceHubConfig;
use crate::text_inject;

pub fn run() -> cosmic::iced::Result {
    cosmic::applet::run::<VoiceHubApplet>(())
}

pub struct VoiceHubApplet {
    core: cosmic::Core,
    config: VoiceHubConfig,
    popup: Option<Id>,
    
    // Estado
    is_recording: bool,
    transcript: String,
    recording_duration: Duration,
    word_count: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleRecording,
    StopRecording,
    CopyToClipboard,
    ClearTranscript,
    InjectText,
    
    TogglePopup,
    PopupClosed(Id),
    
    RecordingTick,
    UpdateConfig(VoiceHubConfig),
}

impl cosmic::Application for VoiceHubApplet {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.deivisan.voicehub";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let config = VoiceHubConfig::load();
        
        let app = VoiceHubApplet {
            core,
            config,
            popup: None,
            is_recording: false,
            transcript: String::new(),
            recording_duration: Duration::ZERO,
            word_count: 0,
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let icon_name = if self.is_recording {
            "media-playback-stop-symbolic"
        } else {
            "audio-input-microphone-symbolic"
        };
        
        self.core
            .applet
            .icon_button(icon_name)
            .on_press(Message::ToggleRecording)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        // Header
        let header = widget::row()
            .push(widget::text("VoiceHub").size(18))
            .push(widget::horizontal_space())
            .push(widget::text(format!("{} palavras", self.word_count)).size(12))
            .align_y(cosmic::iced::Alignment::Center)
            .spacing(12);

        // Área de transcrição
        let transcript_area = widget::scrollable(
            widget::text(&self.transcript).size(14)
        )
        .height(200);

        // Stats bar
        let duration_text = format!(
            "{:02}:{:02}",
            self.recording_duration.as_secs() / 60,
            self.recording_duration.as_secs() % 60
        );
        
        let stats = widget::row()
            .push(widget::text(format!("⏱️ {}", duration_text)).size(12))
            .push(widget::horizontal_space())
            .push(widget::text(format!("🌍 {}", self.config.language)).size(12))
            .align_y(cosmic::iced::Alignment::Center);

        // Botões de ação
        let action_buttons = widget::row()
            .spacing(8)
            .push(
                widget::button::standard("📋 Copiar")
                    .on_press(Message::CopyToClipboard)
            )
            .push(
                widget::button::standard("🗑️ Limpar")
                    .on_press(Message::ClearTranscript)
            )
            .push(
                widget::button::standard("📤 Injetar")
                    .on_press(Message::InjectText)
            );

        // Botão principal de gravação
        let record_button = widget::button::standard(
            if self.is_recording { "⏹️ Parar" } else { "🎤 Gravar" }
        )
        .on_press(Message::ToggleRecording)
        .width(cosmic::iced::Length::Fill);

        // Conteúdo
        let content = widget::column()
            .push(header)
            .push(widget::divider::horizontal::default())
            .push(transcript_area)
            .push(stats)
            .push(action_buttons)
            .push(record_button)
            .spacing(12)
            .padding(16);

        self.core.applet.popup_container(content).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subs = vec![];
        
        // Tick para atualizar duração
        if self.is_recording {
            subs.push(
                cosmic::iced::time::every(Duration::from_secs(1))
                    .map(|_| Message::RecordingTick)
            );
        }
        
        // Watch config changes
        subs.push(
            self.core()
                .watch_config::<VoiceHubConfig>(Self::APP_ID)
                .map(|update| Message::UpdateConfig(update.config))
        );
        
        Subscription::batch(subs)
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::ToggleRecording => {
                if self.is_recording {
                    self.is_recording = false;
                    self.recording_duration = Duration::ZERO;
                } else {
                    self.is_recording = true;
                    self.recording_duration = Duration::ZERO;
                }
                Task::none()
            }
            
            Message::StopRecording => {
                self.is_recording = false;
                self.recording_duration = Duration::ZERO;
                Task::none()
            }
            
            Message::CopyToClipboard => {
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(&self.transcript);
                }
                Task::none()
            }
            
            Message::ClearTranscript => {
                self.transcript.clear();
                self.word_count = 0;
                Task::none()
            }
            
            Message::InjectText => {
                let text = self.transcript.clone();
                Task::perform(
                    async move { text_inject::inject_text(text).await },
                    |_| cosmic::Action::App(Message::StopRecording),
                )
            }
            
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(400.0)
                        .min_width(350.0)
                        .min_height(300.0)
                        .max_height(600.0);
                    get_popup(popup_settings)
                }
            }
            
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
                Task::none()
            }
            
            Message::RecordingTick => {
                if self.is_recording {
                    self.recording_duration += Duration::from_secs(1);
                }
                Task::none()
            }
            
            Message::UpdateConfig(config) => {
                self.config = config;
                Task::none()
            }
        }
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
