use bevy::prelude::*;

use super::letter::Letter;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, load_letters);
}

#[derive(Resource)]
pub struct LetterList {
    pub letters: Vec<Letter>,
}

fn load_letters(mut commands: Commands) {
    commands.insert_resource(LetterList {
        letters: vec![
            //	english
            Letter {
                value: "a".into(),
                asset_path: "a.png".into(),
                sound_path: "a.ogg".into(),
                sound_duration: 0.03,
            },
            Letter {
                value: "b".into(),
                asset_path: "b.png".into(),
                sound_path: "b.ogg".into(),
                sound_duration: 0.04,
            },
            Letter {
                value: "c".into(),
                asset_path: "c.png".into(),
                sound_path: "c.ogg".into(),
                sound_duration: 0.014,
            },
            Letter {
                value: "d".into(),
                asset_path: "d.png".into(),
                sound_path: "d.ogg".into(),
                sound_duration: 0.04,
            },
            Letter {
                value: "e".into(),
                asset_path: "e.png".into(),
                sound_path: "e.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "f".into(),
                asset_path: "f.png".into(),
                sound_path: "f.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "g".into(),
                asset_path: "g.png".into(),
                sound_path: "g.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "h".into(),
                asset_path: "h.png".into(),
                sound_path: "h.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "i".into(),
                asset_path: "i.png".into(),
                sound_path: "i.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "j".into(),
                asset_path: "j.png".into(),
                sound_path: "j.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "k".into(),
                asset_path: "k.png".into(),
                sound_path: "k.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "l".into(),
                asset_path: "l.png".into(),
                sound_path: "l.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "m".into(),
                asset_path: "m.png".into(),
                sound_path: "m.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "n".into(),
                asset_path: "n.png".into(),
                sound_path: "n.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "o".into(),
                asset_path: "o.png".into(),
                sound_path: "o.ogg".into(),
                sound_duration: 0.3,
            },
            Letter {
                value: "p".into(),
                asset_path: "p.png".into(),
                sound_path: "p.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "q".into(),
                asset_path: "q.png".into(),
                sound_path: "q.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "r".into(),
                asset_path: "r.png".into(),
                sound_path: "r.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "s".into(),
                asset_path: "s.png".into(),
                sound_path: "s.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "t".into(),
                asset_path: "t.png".into(),
                sound_path: "t.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "u".into(),
                asset_path: "u.png".into(),
                sound_path: "u.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "v".into(),
                asset_path: "v.png".into(),
                sound_path: "v.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "w".into(),
                asset_path: "w.png".into(),
                sound_path: "w.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "x".into(),
                asset_path: "x.png".into(),
                sound_path: "x.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "y".into(),
                asset_path: "y.png".into(),
                sound_path: "y.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "z".into(),
                asset_path: "z.png".into(),
                sound_path: "z.ogg".into(),
                sound_duration: 0.0,
            },
            //	nahuatl
            Letter {
                value: "tl".into(),
                asset_path: "tl.png".into(),
                sound_path: "tl.ogg".into(),
                sound_duration: 0.0,
            },
            //	svenska
            Letter {
                value: "å".into(),
                asset_path: "å.png".into(),
                sound_path: "å.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "ä".into(),
                asset_path: "ä.png".into(),
                sound_path: "ä.ogg".into(),
                sound_duration: 0.0,
            },
            Letter {
                value: "ö".into(),
                asset_path: "ö.png".into(),
                sound_path: "ö.ogg".into(),
                sound_duration: 0.0,
            },
        ],
    });
}
