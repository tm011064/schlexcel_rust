extern crate clap;
extern crate rand;

use clap::{ Arg, App };
use rand::Rng;

use std::path::Path;
use std::fs;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::ffi::OsString;

const JOKES: [&'static str; 214] = [
      "  Q: What does a nosey pepper do?\n  A: Gets jalapeno business! ",
      "  Q: What do you call a fake noodle?\n  A: An Impasta ",
      "  Q: What do you call an alligator in a vest?\n  A: An Investigator ",
      "  Q: What happens if you eat yeast and shoe polish?\n  A: Every morning you'll rise and shine! ",
      "  Q: \"What's the difference between a guitar and a fish?\"\n  A: \"You can't tuna fish.\" ",
      "  Q: Did you hear about the race between the lettuce and the tomato?\n  A: The lettuce was a \"head\" and the tomato was trying to \"ketchup\"! ",
      "  Q: What is it called when a cat wins a dog show?\n  A: A CAT-HAS-TROPHY! ",
      "  Q: Why can't you give Elsa a balloon?\n  A: Because she will Let it go. ",
      "  Q: What do you get from a pampered cow?\n  A: Spoiled milk. ",
      "  Q: What do lawyers wear to court?\n  A: Lawsuits! ",
      "  Q: What gets wetter the more it dries?\n  A: A towel. ",
      "  Q: What do you get if you cross a cat with a dark horse?\n  A: Kitty Perry ",
      "  Q: What did the pencile say to the other pencil?\n  A: your looking sharp. ",
      "  Q: What did Bacon say to Tomato?\n  A: Lettuce get together! ",
      "  Q: What is the most hardworking part of the eye?\n  A: the pupil ",
      "  Q: How do you make a tissue dance?\n  A: Put a little boogey in it! ",
      "  Q: Why did the picture go to jail?\n  A: Because it was framed. ",
      "  Q: What do you call security guards working outside Samsung shops?\n  A: Guardians of the Galaxy. ",
      "  Q: What do you get when you cross fish and an elephant?\n  A: Swimming trunks. ",
      "  Q: Where do bees go to the bathroom?\n  A: At the BP station! ",
      "  Q: What do you call a baby monkey?\n  A: A Chimp off the old block. ",
      "  Q: Who earns a living driving their customers away?\n  A: A taxi driver. ",
      "  Q: \"How do you shoot a killer bee?\"\n  A: \"With a bee bee gun.\" ",
      "  Q: How do you drown a Hipster?\n  A: In the mainstream. ",
      "  Q: How do you make holy water?\n  A: Boil the hell out of it! ",
      "  Q: What happened to the dog that swallowed a firefly?\n  A: It barked with de-light! ",
      "  Q: What stays in the corner and travels all over the world?\n  A: A stamp. ",
      "  Q: Why did the computer go to the doctor?\n  A: Because it had a virus! ",
      "  Q: Why are frogs so happy?\n  A: They eat whatever bugs them",
      "  Q: What do you get when you cross a cow and a duck?\n  A: Milk and quackers! ",
      "  Q: What did the leopard say after eating his owner?\n  A: Man, that hit the \"spot.\" ",
      "  Q: What do you call a sleeping bull?\n  A: A bulldozer! ",
      "  Q: What is the tallest building in the world?\n  A: The library! It has the most stories! ",
      "  Q: What do you call a belt with a watch on it?\n  A: A waist of time ",
      "  Q: Why did the banana go to the Doctor?\n  A: Because it was not peeling well ",
      "  Q: Why is England the wettest country?\n  A: Because the queen has reigned there for years! ",
      "  Q: Why do fish live in salt water?\n  A: Because pepper makes them sneeze! ",
      "  Q: Why did the man put his money in the freezer?\n  A: He wanted cold hard cash! ",
      "  Q: What do you get when you cross a snowman with a vampire?\n  A: Frostbite. ",
      "  Q: What is the best day to go to the beach?\n  A: Sunday, of course! ",
      "  Q: What bow can't be tied?\n  A: A rainbow! ",
      "  Q: What season is it when you are on a trampoline?\n  A: Spring time. ",
      "  Q: Where did the computer go to dance?\n  A: To a disc-o. ",
      "  Q: What has one head, one foot and four legs?\n  A: A Bed ",
      "  Q: What is the difference between a school teacher and a train?\n  A: The teacher says spit your gum out and the train says \"chew chew chew\". ",
      "  Q: Why did the birdie go to the hospital?\n  A: To get a tweetment. ",
      "  Q: What do you call someone who is afraid of Santa?\n  A: A Clausterphobic ",
      "  Q: What sound do porcupines make when they kiss?\n  A: Ouch ",
      "  Q: Why was the guy looking for fast food on his friend?\n  A: Because his friend said dinner is on me. ",
      "  Q: Why is a 2016 calendar more popular than a 2015 calendar?\n  A: It has more dates. ",
      "  Q: Did you hear the joke about the roof?\n  A: Never mind, it's over your head! ",
      "  Q: What is brown and has a head and a tail but no legs?\n  A: A penny. ",
      "  Q: Why didn't the skeleton go to the dance?\n  A: Because he had no-body to go with. ",
      "  Q: How do crazy people go through the forest?\n  A: They take the psycho path. ",
      "  Q: What three candies can you find in every school?\n  A: Nerds, DumDums, and smarties. ",
      "  Q: Why are pirates called pirates?\n  A: Cause they arrrrr. ",
      "  Q: What do prisoners use to call each other?\n  A: Cell phones. ",
      "  Q: Where do snowmen keep their money?\n  A: In snow banks. ",
      "  Q: What washes up on very small beaches?\n  A: Microwaves!    ",
      "  Q: What goes through towns, up & over hills, but doesn't move?\n  A: The road! ",
      "  Q: Why was there thunder and lightning in the lab?\n  A: The scientists were brainstorming! ",
      "  Q: Why did Tony go out with a prune?\n  A: Because he couldn't find a date! ",
      "  Q: What did the little mountain say to the big mountain?\n  A: Hi Cliff! ",
      "  Q: What did Winnie The Pooh say to his agent?\n  A: Show me the honey! ",
      "  Q: What do you call a funny mountain?\n  A: hill-arious ",
      "  Q: What did the candle say to the other candle?\n  A: I'm going out tonight. ",
      "  Q: Why couldn't the pirate play cards?\n  A: Because he was sitting on the deck! ",
      "  Q: What did the janitor say when he jumped out of the closet?\n  A: (SUPPLIES!) ",
      "  Q: Why did the traffic light turn red?\n  A: You would too if you had to change in the middle of the street! ",
      "  Q: What did one elevator say to the other elevator?\n  A: I think I'm coming down with something! ",
      "  Q: What do you say when you lose a wii game?\n  A: I want a wii-match! ",
      "  Q: What never asks questions but receives a lot of answers?\n  A: the Telephone. ",
      "  Q: How do you make an Octupus laugh?\n  A: With ten-tickles ",
      "  Q: Why can't your nose be 12 inches long?\n  A: Because then it would be a foot! ",
      "  Q: What has four wheels and flies?\n  A: A garbage truck! ",
      "  Q: What starts with a P, ends with an E, and has a million letters in it?\n  A: Post Office! ",
      "  Q: What did the blanket say to the bed?\n  A: Don't worry, I've got you covered! ",
      "  Q: Why should you take a pencil to bed?\n  A: To draw the curtains! ",
      "  Q: How many books can you put in an empty backpack?\n  A: One! After that its not empty! ",
      "  Q: What kind of flower doesn't sleep at night?\n  A: The Day-zzz ",
      "  Q: Did you hear they're changing the flooring in daycare centers?\n  A: They're calling it infant-tile! ",
      "  Q: What kind of button won't unbutton?\n  A: A bellybutton! ",
      "  Q: What did the triangle say to the circle?\n  A: Your pointless! ",
      "  Q: Why do sea-gulls fly over the sea?\n  A: Because if they flew over the bay they would be bagels! ",
      "  Q: What dog keeps the best time?\n  A: A watch dog. ",
      "  Q: What did the man say to the wall?\n  A: One more crack like that and I'll plaster ya! ",
      "  Q: Why did the tomato turn red?\n  A: It saw the salad dressing! ",
      "  Q: Why do girls scouts sell cookies?\n  A: They wanna make a sweet first impression. ",
      "  Q: What did the grape do when it got stepped on?\n  A: It let out a little wine! ",
      "  Q: What kind of berry has a coloring book?\n  A: A crayon-berry ",
      "  Q: What did the judge say when the skunk walked in the court room?\n  A: Odor in the court. ",
      "  Q: What did the fish say when he swam into the wall?\n  A: Dam! ",
      "  Q: Why don't skeletons fight each other?\n  A: They don't have the guts. ",
      "  Q: What did the janitor say when he jumped out of the closet?\n  A: SUPPLIES! ",
      "  Q: Why did the scientist go to the tanning salon?\n  A: Because he was a paleontologist. ",
      "  Q: What happened when a faucet, a tomato and lettuce were in a race?\n  A: The lettuce was ahead, the faucet was running and the tomato was trying to ketchup. ",
      "  Q: Why was the student's report card wet?\n  A: It was below C level! ",
      "  Q: How many tickles does it take to make an octopus laugh?\n  A: Tentacles. ",
      "  Q: What did the traffic light say to the car?\n  A: Don't look, I'm changing. ",
      "  Q: What do you call cheese that is not yours?\n  A: Nacho Cheese ",
      "  Q: How do you find a Princess?\n  A: You follow the foot Prince. ",
      "  Q: What streets do ghosts haunt?\n  A: Dead ends! ",
      "  Q: What did the penny say to the other penny?\n  A: We make perfect cents. ",
      "  Q: Why did the man with one hand cross the road?\n  A: To get to the second hand shop. ",
      "  Q: Why did the boy sprinkle sugar on his pillow before he went to sleep?\n  A: So he could have sweet dreams. ",
      "  Q: Why did the robber take a bath?\n  A: Because he wanted to make a clean getaway. ",
      "  Q: What happens if life gives you melons?\n  A: Your dyslexic ",
      "  Q: What music are balloons scared of?\n  A: Pop music ",
      "  Q: What did the judge say to the dentist?\n  A: Do you swear to pull the tooth, the whole tooth and nothing but the tooth. ",
      "  Q: Why did the boy tiptoe past the medicine cabinet?\n  A: He didn't want to wake the sleeping pills! ",
      "  Q: What do you get when you cross a fridge with a radio?\n  A: Cool Music. ",
      "  Q: What goes up when the rain comes down?\n  A: An umbrella. ",
      "  Q: Why did the belt go to jail?\n  A: Because it held up a pair of pants! ",
      "  Q: What happens if life gives you melons?\n  A: Your dyslexic ",
      "  Q: What did the stamp say to the envelope?\n  A: Stick with me and we will go places! ",
      "  Q: What kind of lights did Noah use on the Ark?\n  A: Flood lights! ",
      "  Q: Why don't you see giraffes in elementary school?\n  A: Because they're all in High School! ",
      "  Q: Which is the longest word in the dictionary?\n  A: \"Smiles\", because there is a mile between each \"s\"! ",
      "  Q: Which month do soldiers hate most?\n  A: The month of March! ",
      "  Q: What did the painter say to the wall?\n  A: One more crack like that and I'll plaster you! ",
      "  Q: Why did the computer break up with the internet?\n  A: There was no \"Connection\". ",
      "  Q: Why do golfers wear two pairs of pants?\n  A: In case they get a hole in one! ",
      "  Q: Why can't you take a nap during a race?\n  A: Because if you snooze, you loose! ",
      "  Q: Why did Goofy put a clock under his desk?\n  A: Because he wanted to work over-time! ",
      "  Q: Why did Johnny throw the clock out of the window?\n  A: Because he wanted to see time fly! ",
      "  Q: What do you call a book that's about the brain?\n  A: A mind reader. ",
      "  Q: When do you stop at green and go at red?\n  A: When you're eating a watermelon! ",
      "  Q: Why did God make only one Yogi Bear?\n  A: Because when he tried to make a second one he made a Boo-Boo ",
      "  Q: How did the farmer mend his pants?\n  A: With cabbage patches! ",
      "  Q: Why did the man lose his job at the orange juice factory?\n  A: He couldn't concentrate! ",
      "  Q: How do you repair a broken tomato?\n  A: Tomato Paste! ",
      "  Q: Why did the baby strawberry cry?\n  A: Because his parents were in a jam! ",
      "  Q: What was the Cat in the Hat looking for in the toilet?\n  A: For thing one and thing two. ",
      "  Q: What did the hamburger name his daughter?\n  A: Patty! ",
      "  Q: What kind of egg did the bad chicken lay?\n  A: A deviled egg! ",
      "  Q: What kind of key opens the door on Thanksgiving?\n  A: A turkey! ",
      "  Q: Why did the cookie go to the hospital?\n  A: He felt crummy! ",
      "  Q: Why were the teacher's eyes crossed?\n  A: She couldn't control her pupils! ",
      "  Q: What do you call a guy who never farts in public?\n  A: A private tutor. ",
      "  Q: What do you call a bear with no socks on?\n  A: Bare-foot. ",
      "  Q: What can you serve but never eat?\n  A: A volleyball. ",
      "  Q: What kind of shoes do all spies wear?\n  A: Sneakers. ",
      "  Q: Why did the soccer player bring string to the game?\n  A: So he could tie the score. ",
      "  Q: Why is a baseball team similar to a muffin?\n  A: They both depend on the batter. ",
      "  Q: What did the alien say to the garden?\n  A: Take me to your weeder. ",
      "  Q: Why do watermelons have fancy weddings?\n  A: Because they cantaloupe. ",
      "  Q: Have you heard the joke about the butter?\n  A: I better not tell you, it might spread. ",
      "  Q: How do baseball players stay cool?\n  A: They sit next to their fans. ",
      "  Q: Why was the math book sad?\n  A: Because it had too many problems. ",
      "  Q: What runs but doesn't get anywhere?\n  A: A refrigerator. ",
      "  Q: What is an astronaut's favorite place on a computer?\n  A: The Space bar! ",
      "  Q: What exam do young witches have to pass?\n  A: A spell-ing test! ",
      "  Q: What do you call a sheep with no head and no legs?\n  A: A cloud! ",
      "  Q: Why did the boy eat his homework?\n  A: Because his teacher said it was a piece of cake! ",
      "  Q: Why is Basketball such a messy sport?\n  A: Because you dribble on the floor! ",
      "  Q: How do you communicate with a fish?\n  A: Drop him a line! ",
      "  Q: Where do sheep go to get haircuts?\n  A: To the Baa Baa shop! ",
      "  Q: What does a shark like to eat with peanut butter?\n  A: Jellyfish! ",
      "  Q: What do cats eat for breakfast?\n  A: Mice Crispies! ",
      "  Q: Who goes to the bathroom in the middle of a party?\n  A: A party pooper. ",
      "  Q: Why can't a leopard hide?\n  A: Because he's always spotted! ",
      "  Q: What do you give a dog with a fever?\n  A: Mustard, its the best thing for a hot dog! ",
      "  Q: What do you get when you cross a cat with a lemon?\n  A: A sour puss! ",
      "  Q: Why do birds fly south for the winter?\n  A: Its easier than walking! ",
      "  Q: What did the M&M go to college?\n  A: Because he wanted to be a Smarty. ",
      "  Q: What kind of key opens a banana?\n  A: A monkey! ",
      "  Q: How do you know that carrots are good for your eyesight?\n  A: Have you ever seen a rabbit wearing glasses? ",
      "  Q: Why does a hummingbird hum?\n  A: It doesn't know the words! ",
      "  Q: What do you call a house that likes food?\n  A: a Condoment! ",
      "  Q: Why are some fish at the bottom of the ocean?\n  A: Because they dropped out of school! ",
      "  Q: What do you call a pile of kittens\n  A: a meowntain ",
      "  Q: What goes up and down but doesn't move?\n  A: The temperature! ",
      "  Q: What happened to the wooden car with wooden wheels and wooden engine?\n  A: it wooden go! ",
      "  Q: Which weighs more, a ton of feathers or a ton of bricks?\n  A: Neither, they both weigh a ton! ",
      "  Q: What has one horn and gives milk\n  A: A milk truck. Q. Did you hear about the party a little boy had for his sisters barbie dolls? A. It was a Barbie-Q. ",
      "  Q: Where do bulls get their messages?\n  A: On a bull-etin board. ",
      "  Q: What do bulls do when they go shopping?\n  A: They CHARGE! ",
      "  Q: What runs but can't walk?\n  A: The faucet! ",
      "  Q: Whens the best time to go to the dentist?\n  A: Tooth-hurty ",
      "  Q: What kind of bed does a mermaid sleep in?\n  A: A water bed! ",
      "  Q: What kind of crackers do firemen like in their soup?\n  A: Firecrackers! ",
      "  Q: Why did the barber win the race?\n  A: Because he took a short cut. ",
      "  Q: What's taken before you get it?\n  A: Your picture. ",
      "  Q: What concert costs 45 cents?\n  A: 50 Cent featuring Nickleback. ",
      "  Q: Why did the tree go to the dentist?\n  A: To get a root canal. ",
      "  Q: When I was young there was only 25 letters in the Alphabet?\n  A: Nobody new why. ",
      "  Q: What is it called when a cat wins a dog show?\n  A: A CAT-HAS-TROPHY! ",
      "  Q: What can go up a chimney down, but can't go down a chimney up?\n  A: An umbrella. ",
      "  Q: Why was the broom late?\n  A: It over swept! ",
      "  Q: Why didn't the 11 year old go to the pirate movie?\n  A: because it was rated arrrrr",
      "  Q: What did the Super Nintendo say to the Sega Genesis?\n  A: \"You know, everyone always tells me that I'm a bit better than you.\" ",
      "  Q: What's the difference between Ms. and Mrs.?\n  A: Mr. ",
      "  Q: What word looks the same backwards and upside down?\n  A: Swims ",
      "  Q: Where does a tree store their stuff?\n  A: In there Trunk! ",
      "  Q: What did the nose say to the finger?\n  A: Stop picking on me. ",
      "  Q: What did the tie say to the hat?\n  A: You go on ahead and I'll hang around! ",
      "  Q: Where does bad light go?\n  A: PRISM! ",
      "  Q: What did one plate say to the other?\n  A: Dinners on me ",
      "  Q: Who cleans the bottom of the ocean?\n  A: A Mer-Maid ",
      "  Q: Where do pencils go on vacation?\n  A: Pennsylvania ",
      "  Q: What is heavy forward but not backward?\n  A: Ton. ",
      "  Q: What do you get when you plant kisses?\n  A: Tu-lips (two-lips) ",
      "  Q: What pet makes the loudest noise?\n  A: A trum-pet! ",
      "  Q: What do you call a rabbit with fleas?\n  A: Bugs Bunny! ",
      "  Q: Why did the girl bring lipstick and eye shadow to school?\n  A: She had a make-up exam! ",
      "  Q: What is a bubbles least favorite drink?\n  A: Soda POP ",
      "  Q: What did one eyeball say to the other eyeball?\n  A: Between you and me something smells. ",
      "  Q: What stays on the ground but never gets dirty?\n  A: Shadow. ",
      "  Q: Name a city where no one goes?\n  A: Electricity ",
      "  Q: What four letters will frighten a burglar?\n  A: O I C U ",
      "  Q: What's the difference between a cat and a frog?\n  A: A Cat has nine lives but a Frog croaks every night! ",
      "  Q: Why can you never trust atoms?\n  A: They make up everything! ",
      "  Q: Where does bad light go?\n  A: To prism! ",
      "  Q: I can run but not walk, have a mouth but can't talk, and a bed, but I do not sleep. What am I?\n  A: A River."
];




fn get_random_joke() -> &'static str {
    rand::thread_rng().choose(&JOKES).unwrap()
}

struct FileContent {
    file_name: OsString,
    values: Vec<String>,
}

impl FileContent {
    fn get_file_name(&self) -> OsString { 
        self.file_name.to_os_string()
    }

    fn get_value_at_or_empty(&self, index: usize) -> String {
        if self.values.len() <= index {
            return "".to_string();
        }

        self.values[index].to_string()
    }

    fn get_length(&self) -> usize {
        self.values.len()
    }
}

fn get_file_contents(source_folder: &str) -> Vec<FileContent> {
    let source_directory = fs::read_dir(source_folder).unwrap();

    let mut file_contents = Vec::new();

    for path in source_directory {
        let file_path = path.unwrap().path();

        if file_path.is_dir()
           || file_path.extension().map(|s| s != "fit").unwrap_or(true) {
            continue;
        }

        let file_name = file_path.file_name().unwrap().to_os_string();
        let file = File::open(file_path.display().to_string()).unwrap();

        let mut values = Vec::new();
        let mut second_values = Vec::new();

        for line in BufReader::new(file).lines() {
            let line_string = line.unwrap().to_string();
            let mut split = line_string.split_whitespace();

            let splitted: Vec<&str> = split.collect();
            
            if splitted.len() != 3 {
                continue;
            }

            values.push(splitted[1].to_string());
            second_values.push(splitted[2].to_string());
        }

        values.append(&mut second_values);

        file_contents.push(FileContent { file_name, values });
    }

    file_contents
}

fn main() {
    let matches = App::new("schlexcel")
        .arg(Arg::with_name("sourceFolder")
            .takes_value(true)
            .short("s")
            .long("sourceFolder")
            .help("Sets a custom config file")
            .required(true))
        .arg(Arg::with_name("destinationFolder")
            .short("d")
            .long("destinationFolder")
            .help("Sets a custom config file"))
        .arg(Arg::with_name("destinationFileName")
            .short("f")
            .long("destinationFileName")
            .help("Sets a custom config file"))
        .get_matches();

    let source_folder = matches.value_of("sourceFolder").unwrap();
    let destination_folder = matches.value_of("destinationFolder").unwrap_or(source_folder);
    let destination_file_name = matches.value_of("destinationFileName").unwrap_or("output.csv");

    let file_contents = get_file_contents(source_folder);

    let joined_path = Path::new(".")
        .join(destination_folder)
        .join(destination_file_name);

    let destination_path = match joined_path.to_str() {
        None => panic!("Destination path is not a valid UTF-8 sequence"),
        Some(s) => s,
    };

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(destination_path)
        .unwrap();

    let headers: Vec<_> = file_contents
        .iter()
        .map(|file_conent| file_conent.get_file_name().into_string().unwrap())
        .collect();

    if let Err(e) = writeln!(file, "{}", headers.join(",")) {
        eprintln!("Couldn't write to file: {}", e);
    }

    let max_number_of_values = file_contents
        .iter()
        .map(|file_conent| file_conent.get_length())
        .max()
        .unwrap();
    for i in 0..max_number_of_values {
        let values: Vec<_> = file_contents
            .iter()
            .map(|file_conent| file_conent.get_value_at_or_empty(i))
            .collect();

        if let Err(e) = writeln!(file, "{}", values.join(",")) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    println!("
  WOHOO! File generated at '{}'

     / /
     L_L_
    /    \\
    |00  |       _______
    |_/  |      /  ___  \\
    |    |     /  /   \\  \\
    |    |_____\\  \\_  /  /
     \\          \\____/  /_____
      \\ _______________/______\\.............................
      
{}

  See you next time!
", 
      destination_path,
      get_random_joke());
}
