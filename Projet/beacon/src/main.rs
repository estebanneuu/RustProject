use std::{process::Command, fs::{File, self}, io::Write, ops::Sub, time::Duration, thread::sleep};
use chrono::{ Utc, DateTime};
use rand::Rng;


fn main() {

    apply_timer(|x|spawn_command(call_commande(x)),30,5);
    auto_destruction();

}


/// Préparer une commande de type Command
fn call_commande (commande_string: &str) -> Command{
    let mut commandes: Vec<&str> = commande_string.split(" ").collect();
    commandes.reverse();
    let nom_commande = commandes.pop().unwrap().to_string();
    commandes.reverse();

    let mut commande = Command::new(nom_commande);

    for args in commandes {
        commande.arg(args);
    }

    commande

}


/// But : Récupérer le résultat d'une commande sous forme de chaîne de caractère
fn get_output_from_command_to_string(mut commande: Command) -> String {
    let output = commande.output()
                                .expect("command didnt work")
                                .stdout;
    String::from_utf8(output).unwrap()
}



/// But : Executer une commande sur le terminal 
fn spawn_command(mut commande : Command){
    commande.spawn()
            .expect("commande didnt work");
}





/// But : Créer un script qui execute cet executable et Créer un service qui Executera ce script au démarrage de l'ordinateur 
fn create_shortcut(path: String){
    
    let script_service = b"[Service]\nDescription = Beacon\nType = oneshot\nRemainAfterExit = yes\nExecStart = /etc/init.d/beacon.sh\n\n[Install]\nWantedBy = multi-user.target";


    create_file("src/beacon.sh", path.as_bytes());
    create_file("src/beacon.service", script_service);

}


/// But : Créer un fichier sur le chemin "path" et y écrire le "contenu", renvoie le fichier pour une utilisation future
fn create_file(path: &str,content:&[u8])->File{
    let mut file = File::create(path).unwrap();
    file.write_all(content).unwrap();
    file
}

 
/// But : Regarder si le fichier date.txt existe sinon il le créé ainsi que les script et shortcut de l'executable, renvoie la premiere ligne du fichier date.txt
fn open_or_create_file_date(path:&str,nom_fichier: &str, content:&[u8]) -> String{
    let greeting_file_result = fs::read_to_string(nom_fichier);

    match greeting_file_result {
        Ok(line) => line,
        Err(_error) => {
            let commande = "mkdir ".to_owned() + path;
            spawn_command(call_commande(&commande));
            sleep(Duration::from_secs(2));
            create_file(nom_fichier, content);
            create_shortcut(get_output_from_command_to_string(call_commande("readlink -f target/debug/beacon.exe")));
            open_or_create_file_date(path, nom_fichier,content)
        },
    }
}



/// But : obtenir le nombre de seconde separant les deux dates
fn match_two_date(date_1:String, date_2:String) -> i64{
    let res = DateTime::parse_from_rfc2822(&date_1).unwrap() ;
    let res_2 = DateTime::parse_from_rfc2822(&date_2).unwrap();
    res.sub(res_2).num_seconds()
}


/// But : obtenir le nombre de seconde separant la date actuel et la date présente dans date.txt
fn get_last_connexion() -> i64 {
    let now = Utc::now().to_rfc2822();
    let date = open_or_create_file_date("src/toto/","src/toto/date.txt", now.as_bytes()); 
    match_two_date(now, date)
}


/// But : Détruire les fichiers date.txt, beacon.service, beacon.sh, beacon.exe
fn auto_destruction(){
    let path = "src/toto";
    let commande = "rm -r ".to_owned() + path;
    spawn_command(call_commande(&commande));
    spawn_command(call_commande("rm src/beacon.service"));
    spawn_command(call_commande("rm src/beacon.sh"));
    spawn_command(call_commande("rm target/debug/beacon.exe"));
}


/// But : appliquer une fonction qui s'execute toute les "intervals" secondes jusqu'à atteindre max
fn apply_timer<F>(f:F, max:i64, interval: u64) where
    F:Fn(&str) {
        let mut n = get_last_connexion();

        while n <= max {
            sleep(Duration::from_secs(interval));
            n = get_last_connexion();
            let m: i32 = rand::thread_rng().gen_range(1..5);
    
            let commande = match m {
                1=> {
                    "ls -l -a"
                },
                2=> {
                    "touch toto.txt"
                },
                3=> {
                    "rm toto.txt"
                },
                _ => {
                    "echo 'toto"
                }
                
            };
            f(commande)
        }
}