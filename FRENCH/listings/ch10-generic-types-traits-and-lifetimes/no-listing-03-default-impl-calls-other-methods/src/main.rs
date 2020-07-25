use chapter10::{self, Resumable, Tweet};

fn main() {
    // ANCHOR: here
    let tweet = Tweet {
        nom_utilisateur: String::from("jean"),
        contenu: String::from("Bien sûr, les amis, comme vous le savez probablement déjà"),
        reponse: false,
        retweet: false,
    };
    
    println!("1 nouveau tweet : {}", tweet.resumer());
    // ANCHOR_END: here
}
