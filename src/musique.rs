use rand::Rng;

struct Rythme {
    blanche: i32,
    noire_pointée: i32,
    noire: i32,
    croche_pointée: i32,
    croche: i32,
    double: i32,
}

impl Rythme {
    fn tirer_note(&self) -> &'static str {
        let total_poids = self.blanche + self.noire_pointée + self.noire + self.croche_pointée + self.croche + self.double;
        let mut rng = rand::thread_rng();
        let tirage: i32 = rng.gen_range(0..total_poids);
        let mut somme = 0;

        if tirage < self.blanche {
            return "blanche";
        }
        somme += self.blanche;

        if tirage < somme + self.noire_pointée {
            return "noire_pointée";
        }
        somme += self.noire_pointée;

        if tirage < somme + self.noire {
            return "noire";
        }
        somme += self.noire;

        if tirage < somme + self.croche_pointée {
            return "croche_pointée";
        }
        somme += self.croche_pointée;

        if tirage < somme + self.croche {
            return "croche";
        }

        "double"
    }
}

struct Note {
    durée: String,
    hauteur: String,
}

fn génération_mélodie(durée_totale: i32, tonalité: Vec<&str>, rythme: Rythme) -> Vec<Note> {
    let mut nb_temps = 0;
    let mut suite_notes: Vec<Note> = vec![];
    let tonalité_len = tonalité.len();
    while nb_temps < durée_totale * 16 {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..tonalité_len);
        let note_durée = rythme.tirer_note();
        let note = Note {
            durée: note_durée.to_string(),
            hauteur: tonalité[index].to_string(),
        };
        suite_notes.push(note);
        nb_temps += match note_durée {
            "blanche" => 8,
            "noire_pointée" => 6,
            "noire" => 4,
            "croche_pointée" => 3,
            "croche" => 2,
            "double" => 1,
            _ => 0,
        };
    }
    suite_notes
}

fn note_to_frequency(note: &str) -> f32 {
    match note {
        "do" => 261.63,
        "do#" => 277.18,
        "ré" => 293.66,
        "ré#" => 311.13,
        "mi" => 329.63,
        "fa" => 349.23,
        "fa#" => 369.99,
        "sol" => 392.00,
        "sol#" => 415.30,
        "la" => 440.00,
        "la#" => 466.16,
        "si" => 493.88,
        _ => 440.0,
    }
}

fn gamme_majeure(tonique: &str) -> Vec<&str> {
    match tonique {
        "do" => vec!["do", "ré", "mi", "fa", "sol", "la", "si"],
        "do#" => vec!["do#", "ré#", "mi#", "fa#", "sol#", "la#", "si#"],
        "ré" => vec!["ré", "mi", "fa#", "sol", "la", "si", "do#"],
        "ré#" => vec!["ré#", "mi#", "fa##", "sol#", "la#", "si#", "do##"],
        "mi" => vec!["mi", "fa#", "sol#", "la", "si", "do#", "ré#"],
        "fa" => vec!["fa", "sol", "la", "si♭", "do", "ré", "mi"],
        "fa#" => vec!["fa#", "sol#", "la#", "si", "do#", "ré#", "mi#"],
        "sol" => vec!["sol", "la", "si", "do", "ré", "mi", "fa#"],
        "sol#" => vec!["sol#", "la#", "si#", "do#", "ré#", "mi#", "fa##"],
        "la" => vec!["la", "si", "do#", "ré", "mi", "fa#", "sol#"],
        "la#" => vec!["la#", "si#", "do##", "ré#", "mi#", "fa##", "sol##"],
        "si" => vec!["si", "do#", "ré#", "mi", "fa#", "sol#", "la#"],
        _ => vec!["do", "ré", "mi", "fa", "sol", "la", "si"],
    }
}

fn gamme_mineure(tonique: &str) -> Vec<&str> {
    match tonique {
        "la" => vec!["la", "si", "do", "ré", "mi", "fa", "sol"],
        "la#" => vec!["la#", "si#", "do#", "ré#", "mi#", "fa#", "sol#"],
        "si" => vec!["si", "do#", "ré#", "mi", "fa#", "sol#", "la#"],
        "do" => vec!["do", "ré", "mib", "fa", "sol", "lab", "sib"],
        "do#" => vec!["do#", "ré#", "mi", "fa#", "sol#", "la", "si"],
        "ré" => vec!["ré", "mi", "fa", "sol", "la", "sib", "do"],
        "ré#" => vec!["ré#", "mi#", "fa#", "sol#", "la#", "si", "do#"],
        "mi" => vec!["mi", "fa#", "sol", "la", "si", "do", "ré"],
        "fa" => vec!["fa", "sol", "lab", "sib", "do", "réb", "mib"],
        "fa#" => vec!["fa#", "sol#", "la", "si", "do#", "ré", "mi"],
        "sol" => vec!["sol", "la", "sib", "do", "ré", "mib", "fa"],
        "sol#" => vec!["sol#", "la#", "si", "do#", "ré#", "mi", "fa#"],
        _ => vec!["la", "si", "do", "ré", "mi", "fa", "sol"],
    }
}

fn rythme_préréglé(nom: &str) -> Rythme {
    match nom {
        "lent" => Rythme {
            blanche: 50,
            noire_pointée: 20,
            noire: 15,
            croche_pointée: 10,
            croche: 5,
            double: 0,
        },
        "modéré" => Rythme {
            blanche: 25,
            noire_pointée: 20,
            noire: 25,
            croche_pointée: 15,
            croche: 10,
            double: 5,
        },
        "rapide" => Rythme {
            blanche: 10,
            noire_pointée: 15,
            noire: 20,
            croche_pointée: 25,
            croche: 20,
            double: 10,
        },
        _ => Rythme {
            blanche: 25,
            noire_pointée: 12,
            noire: 25,
            croche_pointée: 13,
            croche: 13,
            double: 12,
        },
    }
}

fn generate_wave_with_adsr(frequency: f32, duration: f32, sample_rate: u32) -> Vec<i16> {
    let total_samples = (duration * sample_rate as f32).floor() as usize;
    let mut samples = Vec::with_capacity(total_samples);
    let amplitude = i16::MAX as f32;
    
    let attack = 0.010_f32.min(duration / 4.0);
    let decay = 0.100_f32.min(duration / 4.0);
    let release = 0.500_f32.min(duration / 2.0);
    let sustain_level = 0.7_f32;
    
    let attack_samples = (attack * sample_rate as f32).floor() as usize;
    let decay_samples = (decay * sample_rate as f32).floor() as usize;
    let release_samples = (release * sample_rate as f32).floor() as usize;
    
    let attack_samples = attack_samples.min(total_samples);
    let decay_samples = decay_samples.min(total_samples - attack_samples);
    let release_samples = release_samples.min(total_samples);
    
    let sustain_samples = total_samples.saturating_sub(attack_samples + decay_samples + release_samples);

    for i in 0..total_samples {
        let envelope_amplitude = if i < attack_samples {
            (i as f32 / attack_samples as f32).min(1.0)
        } else if i < attack_samples + decay_samples {
            1.0 - (1.0 - sustain_level) * ((i - attack_samples) as f32 / decay_samples as f32)
        } else if i < attack_samples + decay_samples + sustain_samples {
            sustain_level
        } else {
            sustain_level * (1.0 - ((i - (total_samples - release_samples)) as f32 / release_samples as f32))
        };

        let sample_wave = (i as f32 * frequency * 2.0 * std::f32::consts::PI / sample_rate as f32).sin();
        
        let sample = (amplitude * envelope_amplitude * sample_wave) as i16;
        samples.push(sample);
    }

    samples
}

pub fn exec_music(v_tonality:&str, v_scale:&str, v_duration:i32, v_rhythm:&str) -> Vec<i16>{
    // Récupération des arguments
    let tonality = v_tonality;
    let scale_type = v_scale;
    let duration: i32 = v_duration;
    let rhythm_name = v_rhythm;

    // Initialisaton du sample final à lire, du type rodio::Sample::I16
    let mut samples= Vec::new();

    // Conversion de la tonalité
    let tonalité = if scale_type == "majeur" {
        gamme_majeure(tonality)
    } else {
        gamme_mineure(tonality)
    };

    // Création du rythme
    let rythme = rythme_préréglé(rhythm_name);

    // Génération de la mélodie
    let melody = génération_mélodie(duration, tonalité, rythme);

    for note in melody {
        let frequency = note_to_frequency(&note.hauteur);
        let duration = match note.durée.as_str() {
            "blanche" => 0.5,
            "noire_pointée" => 0.375,
            "noire" => 0.25,
            "croche_pointée" => 0.1875,
            "croche" => 0.125,
            "double" => 0.0625,
            _ => 0.0,
        };

        samples.extend(generate_wave_with_adsr(frequency, duration, 44100));
        
    }


    return samples;

}