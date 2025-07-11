//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod hero;
pub use hero::Hero;


pub mod button;
pub use button::{Button, ButtonSize, ButtonScheme, ButtonType};

pub mod footer;
pub use footer::Footer;

pub mod auth;
pub use auth::Auth;

pub mod input;
pub use input::{TextInput, PasswordInput, NumberInput, DateInput, SelectInput};

pub mod modal;
pub use modal::Modal;