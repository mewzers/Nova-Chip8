// Note: Supported UI languages.
// FR: Langues UI supportees.
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy)]
pub(crate) enum Langue {
    Francais,
    Anglais,
    Espagnol,
    Italien,
    Allemand,
    Portugais,
    Russe,
    Chinois,
    Japonais,
    Coreen,
    Arabe,
    Hindi,
}

impl Langue {
    // Note: Ordered list used to populate language menus.
    // FR: Liste ordonnee utilisee pour remplir les menus de langue.
    pub(crate) const ALL: [Langue; 12] = [
        Langue::Francais,
        Langue::Anglais,
        Langue::Espagnol,
        Langue::Italien,
        Langue::Allemand,
        Langue::Portugais,
        Langue::Russe,
        Langue::Chinois,
        Langue::Japonais,
        Langue::Coreen,
        Langue::Arabe,
        Langue::Hindi,
    ];

    // Note: Human-readable label shown in the UI.
    // FR: Libelle lisible affiche dans l interface.
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Langue::Francais => "FranÃ§ais",
            Langue::Anglais => "English",
            Langue::Espagnol => "EspaÃ±ol",
            Langue::Italien => "Italiano",
            Langue::Allemand => "Deutsch",
            Langue::Portugais => "PortuguÃªs",
            Langue::Russe => "Ð ÑƒÑÑÐºÐ¸Ð¹",
            Langue::Chinois => "ä¸­æ–‡",
            Langue::Japonais => "æ—¥æœ¬èªž",
            Langue::Coreen => "í•œêµ­ì–´",
            Langue::Arabe => "Ø§Ù„Ø¹Ø±Ø¨ÙŠØ©",
            Langue::Hindi => "à¤¹à¤¿à¤¨à¥à¤¦à¥€",
        }
    }
}

// Note: Tabs shown in the settings window.
// FR: Onglets affiches dans la fenetre des parametres.
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub(crate) enum OngletsSettings {
    Emulateur,
    Video,
    Controles,
    Raccourcis,
    Debug,
}

// Note: User-configurable keyboard shortcuts.
// FR: Raccourcis clavier configurables par l utilisateur.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub(crate) struct Raccourcis {
    pub(crate) pause: String,
    pub(crate) reset: String,
    pub(crate) stop: String,
    pub(crate) charger_jeu: String,
    pub(crate) fullscreen: String,
    pub(crate) savestate_1: String,
    pub(crate) savestate_2: String,
    pub(crate) savestate_3: String,
    pub(crate) loadstate_1: String,
    pub(crate) loadstate_2: String,
    pub(crate) loadstate_3: String,
}

impl Default for Raccourcis {
    // Note: Default key mapping used on first launch and reset.
    // FR: Mapping de touches par defaut utilise au premier lancement et lors du reset.
    fn default() -> Self {
        Self {
            pause: "P".into(),
            reset: "R".into(),
            stop: "Echap".into(),
            charger_jeu: "O".into(),
            fullscreen: "F11".into(),
            savestate_1: "F1".into(),
            savestate_2: "F2".into(),
            savestate_3: "F3".into(),
            loadstate_1: "F5".into(),
            loadstate_2: "F6".into(),
            loadstate_3: "F7".into(),
        }
    }
}
