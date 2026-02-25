use clap::{Parser, Subcommand};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)] // Added PartialEq to help find the person
struct Person {
    first_name: String,
    last_name: Option<String>,
    pet_name: Option<String>,
    doctor_name: Option<String>,
}

#[derive(Parser)]
#[command(name = "RWA_Manager", version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new soul to the archive
    Add {
        #[arg(short, long)] // This allows --first-name or -f
        first_name: String,

        #[arg(short, long)] // This allows --last-name or -l
        last_name: Option<String>,

        #[arg(short, long)] // This allows --pet-name or -p
        pet_name: Option<String>,
        #[arg(short, long)] // This allows --doctor-name or -d
        doctor_name: Option<String>,
    },
    List,
    Delete { 
        #[arg(short, long)]
        name: String 
    },
}

fn main() {
    let cli = Cli::parse();
    
    // We open the archive once at the start of our journey
    let mut db = PickleDb::load(
        "mydata.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json
    ).unwrap_or_else(|_| PickleDb::new("mydata.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json));

   match cli.command {
        // We destructure the fields from the command
        Commands::Add { first_name, last_name, pet_name, doctor_name } => {
            // Because the variable names match the struct fields, 
            // we can use this concise initialization.
            let person = Person { 
                first_name, 
                last_name, 
                pet_name,
                doctor_name
            };
            save_to_archive(&mut db, person);
        }
        Commands::List => {
            list_archive(&db);
        }
        Commands::Delete { name } => {
            delete_from_archive(&mut db, &name);
        }
    }
}

// --- The Keeper's Functions ---

fn save_to_archive(db: &mut PickleDb, person: Person) {
    let mut history: Vec<Person> = db.get("all_persons").unwrap_or_default();
    println!("📖 Writing {} into the Great Chronicle...", person.first_name);
    history.push(person);
    db.set("all_persons", &history).expect("Failed to write to the scroll.");
}

fn list_archive(db: &PickleDb) {
    let history: Vec<Person> = db.get("all_persons").unwrap_or_default();
    if history.is_empty() {
        println!("📭 The Chronicle is empty. No souls have been registered.");
    } else {
        println!("📜 --- The Great Chronicle ---");
        for (i, p) in history.iter().enumerate() {
            println!("{}. {} {} (Pet: {:?}) (Doctor: {:?})", i + 1, p.first_name, p.last_name.as_deref().unwrap_or(""), p.pet_name, p.doctor_name);
        }
    }
}

fn delete_from_archive(db: &mut PickleDb, name: &str) {
    let mut history: Vec<Person> = db.get("all_persons").unwrap_or_default();
    let original_len = history.len();

    // The Banishment: We filter the list, keeping only those who DON'T match the name
    history.retain(|p| p.first_name.to_lowercase() != name.to_lowercase());

    if history.len() < original_len {
        db.set("all_persons", &history).expect("Failed to update the scroll.");
        println!("🔥 {} has been removed from the Chronicle.", name);
    } else {
        println!("❓ No one named '{}' was found in the records.", name);
    }
}