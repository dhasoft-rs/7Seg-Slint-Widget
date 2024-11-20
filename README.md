 # TFRT 02 - 7Seg-Slint-Widget

   _For english version of this document, see README.en.md file_

Ce projet vise à la création d'un Custom Widget "afficheur 7 segments". L'ensemble des éléments qui vont constituer l'IHM sont définis dans la fichier app-window.slint.

<p align="center">
  <img width="250" src="/7SegWidget.png">
  &nbsp;
  <img width="250" src="/7SegEdition.png">
</p>

L'application contient 2 boutons (-) et (+), l'afficheur 3x7segments, ainsi qu'un TextInput permettant de définir directement la valeur à afficher.

<p>&nbsp;</p>

# Principe

L'afficheur va afficher la valeur d"un compteur interne, entre 0 et 999.Il est constitué de 3 images (centaines, dizaines et unités) dont la source va changer selon la valeur du compteur. La valeur du compteur (qui au départ vaut 489) peut être soit modifiée avec les boutons, soit directement être éditée en cliquant sur l'afficheur, puis validée en appuyant sur _[ENTRÉE]_.

<p>&nbsp;</p>

# Constitution du fichier app-window.slint

- Les boutons sont définis par le composant **MyButton**.
- L'afficheur ainsi que le TextInput sont définis dans le composant **MyDisplay**
- Le composant **ImgData** contient les images des 10 chiffres 
- Le composant principal **MainApp** va placer le bouton (-), l'afficheur et le bouton (+) dans une layout horizontale.
...to be continued...


# Le projet

Voici un 1er exemple de code pour réaliser une IHM toute simple avec Slint. Il est issu du template que Slint fournit pour démarrer 'en douceur'. Cette application contient un label qui affiche la valeur d'un compteur, et 2 boutons permettant d'incrémenter ou de décrémenter ce compteur.





# Détails / description des fichiers 

## Fichier app-window.slint :
 - la ligne ```import...``` contient les imports nécessaires, selon les widgets utilisés.
 - le mot-clé export de la ligne ```export component AppWindow inherits Window``` permet au reste de l'application d'accéder au composant AppWindow (de type
   'Window').
 - on définit la taille min de la fenêtre avec ```min-width :``` et ```min-height :```. Attention, on doit spécifier l'unité à utiliser (généralement px, pixels logiques,
   mais il existe aussi les pixels physiques, notés phx).
 - pour les positionnements et dimensionnements, voir => https://docs.slint.dev/latest/docs/slint/src/language/concepts/layouting).
 - on définit le titre et l'image de l'application avec les propriétés 'title' et 'icon'.
 - la ligne ```in-out property <int> counter: 42;``` définit la variable counter comme étant accessible en écriture (partie 'in', appel à ```set_counter()```
   dans le fichier main.rs) et en lecture (partie 'out', appel à ```get_counter()``` dans le fichier main.rs) par l'utilisateur du composant.
 - les lignes ```callback...``` déclarent l'existence de fonctions dont le corps devra être renseigné dans la partie utilisateur du composant (ici, dans le
   fichier main.rs).
 - ces fonctions 'callback' sont appelées à partir d'actions sur les widgets dans le fichier '.slint'. Elles sont ici appelées sur l'évènement 'clicked' des
   boutons 'Decrease value' et 'Increase value'.
 - le widget de type 'Text' affiche la valeur de la variable 'counter' mis à jour par les appels aux fonctions ```request-increase-value()``` et ```request-
   decrease-value()```
 - Slint créé automatiquement les getters et setters nécessaires pour les variables définies dans le fichier '.slint', ainsi que les fontions 'on_XXXXXXXXX'
   déclarées aux lignes 'callback'.
 - les widgets sont définis ensuite dans une 'VerticalBox' afin d'être positionnés de haut en bas dans la fenêtre.


## Fichier buid.rs :
 - Ce fichier est utilisé pour la compilation du (ou des) fichiers .slint. La compilation sera effectuée selon les commandes présentes dans la fonction main() de
   ce fichier.
 - Il existe à la base deux approches pour compiler une IHM Slint :

   **a/ La première méthode (la plus simple) utilise la commande suivante :**
   ```
   slint_build::compile("ui/app-window.slint").expect("Slint build failed");
   ```

      Cette méthode semble être la plus adaptée pour générer une IHM sans utiliser de thème particulier et pour utiliser des widgets 'custom', que nous verrons
      dans un autre mini-projet.

   **b/ La seconde méthode permet de sélectionner un style à appliquer à tous les widgets.**
   
      Les styles disponibles sont ici : https://docs.slint.dev/latest/docs/slint/src/advanced/style

      Les commandes à appliquer sont (pour le style 'fluent-dark') :
      ```
    let config = slint_build::CompilerConfiguration::new().with_style("fluent-dark".into());
    slint_build::compile_with_config("ui/app-window.slint", config).unwrap();
      ```
   
## Fichier main.rs :
  - ce fichier contient le code principal de l'application.    
  - il contient l'import ``std::error::Error`` nécessaire à la gestion d'erreur possiblement renvoyée en sortie de la fonction main() ```(Box<dyn Error>)```
  - il contient aussi la macro
    ```
    slint::include_modules!();
    ```
    nécessaire à l'utilisation du fichier build.rs
    
  - il contient enfin une unique fonction main() qui va commencer par créer notre fenêtre d'application :
    ```
    let ui = AppWindow::new()?;
    ```
    
  - si l'application est correctement créée (pas de sortie en erreur), on définit alors le contenu des méthodes de callback qui ont été déclarées dans les lignes
    'callback' du fichier '.slint' :
    ```
	ui.on_request_increase_value({...});
	ui.on_request_decrease_value({...});
    ```

  - on appelle ensuite la méthode run() de l'application fenêtrée : ```ui.run()?;```   
  - enfin on retourne ```Ok(())```, car main() doit retourner quelque chose de type Result<>


Si vous avez cloné ce projet, il ne vous reste plus qu'à faire un ```cargo build``` pour voir le résultat. Personnellement, j'utilise plus souvent ``cargo run --release``, afin d'obtenir directement un exécutable plus léger.

Voilà, c'est tout pour ce premier Topic, en espérant qu'il aura pu vous être utile...
