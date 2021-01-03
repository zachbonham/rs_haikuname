use rand::Rng;
use clap::{Arg, App};

fn main() {

    let adjectives = [
        "red", "green", "yellow", "orange", "azure", "sapphire", "gold", "silver", "bronze",
        "copper", "rusty", "steely", "lavender", "jasmine", "smokey", "thicc", 
        "aged", "ancient", "autumn", "billowing", "bitter", "black", "blue", "bold",
        "broad", "broken", "calm", "cold", "cool", "crazy", "crimson", "curly", "damp",
        "dark", "dawn", "delicate", "divine", "dry", "empty", "falling", "fancy",
        "flat", "floral", "fragrant", "frosty", "gentle", "green", "hidden", "holy",
        "hazy", "smelly", "dazzling", "flabby", "elegant", "fancy", "glamorous", 
        "gorgeous", "handsome", "scruffy", "unkempt", "agreeable", "ambitious", "delightful",
        "eager", "faithful","kind","polite","silly","thankful","witty","wonderful",
        "colossal","gigantic", "mammoth", "petite", "puny", "short", "teeny",
        "icy", "jolly", "late", "lingering", "little", "lively", "long", "lucky",
        "misty", "morning", "muddy", "mute", "nameless", "noisy", "odd", "old",
        "orange", "patient", "plain", "polished", "proud", "purple", "quiet", "rapid",
        "raspy", "red", "restless", "rough", "round", "royal", "shiny", "shrill",
        "shy", "silent", "small", "snowy", "soft", "solitary", "sparkling", "spring",
        "square", "steep", "still", "summer", "super", "sweet", "throbbing", "tight",
        "tiny", "twilight", "wandering", "weathered", "white", "wild", "winter", "wispy",
        "withered", "yellow", "young"
    ];

    let nouns = [
        "art", "ape", "band", "bar", "base", "bird", "block", "boat", "bonus",
        "bread", "breeze", "brook", "buffalo", "bush", "butterfly", "cake", "cat", "cougar", "cell", "cherry",
        "cloud", "credit", "cobra", "darkness", "dog", "dragon", "dawn", "dew", "disk", "dream", "dust",
        "feather", "field", "fire", "firefly", "flower", "fog", "forest", "frog",
        "frost", "gazebo", "glade", "glitter", "grass", "hall", "hat", "haze", "heart",
        "hill", "king", "lab", "lake", "leaf", "limit", "math", "meadow",
        "mode", "moon", "morning", "mountain", "mouse", "mud", "night", "paper",
        "pine", "poetry", "pond", "queen", "rain", "recipe", "resonance", "rhino", "rice",
        "river", "salad", "scene", "sea", "shadow", "shape", "silence", "sky",
        "smoke", "snake", "snow", "snowflake", "sound", "star", "sun", "sun", "sunset",
        "surf", "thunder", "tooth", "tree", "truth", "union",
        "violet", "voice", "water", "waterfall", "wave", "wildflower", "wind", "wood",
        "minotaur", "goblin", "orc", "elf", "dwarf", "kobold", "angel", "eagle", "bugbear",
        "chimera", "demon", "ape", "badger", "bat", "shark", "wolf", "wolverine", "dryad",
        "harpy", "hydra","kraken","werewolf", "giant", "elemental", "genie", "djinn", "ghost",
        "gnoll", "golem", "gorgon", "mummy", "nymph","ogre", "ooze", "owlbear", "pegasus", "phantom",
        "salamander", "shadow", "scorpion","skeleton", "spectre", "sprite", "titan", "troll", "unicorn",
        "vampire", "wight", "willow-wisp", "wraith", "wyvern", "zombie", "bear", "elephant", "hawk", "horse",
        "leopard", "lion", "lizard", "octopus", "owl", "rat", "raven", "shark", "toad", "tiger", "weasel", "orca",
        "apple", "banana", "mango", "kiwi", "cantaloupe", "potato", "strawberry", "blueberry", "grape", "lemon", "lime",
        "orange", "avocado", "onion", "cherry", "pineapple", "peach", "papaya", "apricot", "persimmon", "coconut"
    ];

    let matches = App::new("Haiku Name")
        .version("0.1.0")
        .author("Zach Bonham <zachbonham@gmail.com>")
        .about("Generates a 'haiku-lik' name ala heroku. e.g. weeping-willow")
        .arg(Arg::new("random")
            .short('r')
            .long("random")
            .value_name("COUNT")
            .about("Specifies if appending a random instance count to the name generated. e.g. weeping-willow-123456")
            .takes_value(true))
        .get_matches();


    let mut rng = rand::thread_rng();
    let adjective_index = rng.gen_range(0..adjectives.len());
    let noun_index = rng.gen_range(0..nouns.len());
    let haiku_name = format!("{}-{}", adjectives[adjective_index], nouns[noun_index]);
    

    if let Some(count) = matches.value_of("random") {
        println!("A random instance count with a length of {} requested", count);

        match count.parse::<i8>() {
            Ok(i) => {
                let random_str = random_str(i);
                let instanced_name = format!("{}-{}", haiku_name, random_str);

                println!("{}", instanced_name);
            },
            Err(_e) => (),
          };

        return ;
    }
    
    println!("{}", haiku_name);
}

fn random_str(len:i8) -> String {

    let mut rng = rand::thread_rng();
    let elements:&str = "012356789abcdef";
    let mut random_str = String::with_capacity(len as usize);

    for _ in 0..len {
        let rng_index = rng.gen_range(0..elements.chars().count());
        random_str.push(elements.chars().nth(rng_index).unwrap());
    }

    random_str
}




