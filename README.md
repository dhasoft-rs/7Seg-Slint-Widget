 # TFRT 02 - 7Seg-Slint-Widget

   _For english version of this document, see README.en.md file_

Ce projet vise à la création d'un Custom Widget "afficheur 7 segments". L'ensemble des éléments qui vont constituer l'IHM sont définis dans la fichier app-window.slint.

<p align="center">
  <img width="250" src="/7SegWidget.png">
  &nbsp; &nbsp; &nbsp;
  <img width="250" src="/7SegEdition.png">
</p>

L'application contient 2 boutons (-) et (+), l'afficheur 3x7segments, ainsi qu'un TextInput permettant de définir directement la valeur à afficher.

<p>&nbsp;</p>

# Principe

L'afficheur va afficher la valeur d"un compteur interne, entre 0 et 999. Il est constitué de 3 images (centaines, dizaines et unités) dont la source va changer selon la valeur du compteur. La valeur du compteur (qui au départ vaut 489) peut être soit modifiée avec les boutons, soit directement être éditée en cliquant sur l'afficheur, puis validée en appuyant sur _[ENTRÉE]_.

<p>&nbsp;</p>

# Constitution du fichier app-window.slint

- Les boutons sont définis par le composant **MyButton**.
- L'afficheur (3 images d'un 7 segments) ainsi que le TextInput sont définis dans le composant **MyDisplay**.
- Le composant **ImgData** contient les images des 10 chiffres. Ça n'est pas un composant graphique à proprement parler, mais plutôt un 'tableau' d'images.
- Le composant principal **AppWindow** va placer le bouton (-), l'afficheur et le bouton (+) dans une layout horizontale.

<p>&nbsp;</p>

# Fonctionnement

Le déroulement global du programme est visible dans la partie HorizontalLayout du composant **AppWindow**.
- on y trouve d'abord le bouton  'decButton', qui va décrémenter le compteur sur son évènement _'clicked'_.
- vient ensuite, le composant 'disp' qui contient l'afficheur et le TextInput permettant de modifier la valeur du compteur. On y trouve la mise à jour des images à utiliser en fonction de la valeur du compteur, la gestion de l'affichage du TextInput sur l'évènement _'clicked'_, la gestion du filtrage des caractères entrés sur l'évènement _'entry_edited'_, et la gestion de la validation de la valeur entrée sur l'évènement _'entry_accepted'_. 
- on trouve enfin le bouton  'incButton', qui va incrémenter le compteur sur son évènement _'clicked'_.
<p>&nbsp;</p>

Le fichier main.rs contient les fonctions appelées lors des évènements décris juste ci-dessus.
Si vous avez besoin de plus d'infos 'basiques' sur la façon de concevoir un projet avec Slint, et sur les liens existant entre la partie Slint (fichier *.slint) et le code Rust (fichier main.rs), je vous invite à aller regarder le premier projet de cette série, que vous trouverez ici : https://github.com/dhasoft-rs/Introducing-Slint/blob/main/ui/app-window.slint .

Voilà, c'est tout pour ce second Topic, enjoy :-)
