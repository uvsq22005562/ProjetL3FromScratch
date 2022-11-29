# Projet Medman
###### Jules Marty - 22005562
#### Utilisation sans arguments : 
> On lance le programme avec la commande  
> 
> > cargo run  
> 
> vont ensuite s'afficher les options (scan, search, playlist), chacune correspondant
> a un numéro.  
> Attention : lorsque vous ferez votre choix, si le programme n'a jamais été lancé
> sur votre machine, la fonction search() pourrait ne pas fonctionner, dans la mesure
> ou le fichier json dans lequel les données sont stockées n'est pas encore créé.  
> Il est donc vivement recommandé de lancer une fois la fonction scan ou playlist.
> ___
> Le cas de scan :  
> il vous sera demandé un chemin vers un fichier. Un fichier test est contenu dans le
> projet, vous pourrez donc vous en servir. Vous avez la possibilité de donner au
> programme un chemin relatif comme absolu. 
> 
> > src/test
> 
> Le programme vous renverra le nombre de fichiers trouvé dans les dossiers et écrira
> dans un fichier json des informations sur chacun d'entre eux.  
> Attention : tout les fichiers écrits par le programme réside dans le dossier "output"
> contenu dans le dossier "src".
> ___
> le cas de search :  
> le programme va ici vous demander une contrainte, pour l'instant et je pense jusqu'au moment
> du rendu de ce projet, le programme ne peut gérer qu'une contrainte unique.  
> voici les différents types de contraintes existants :  
> * album
> * artist
> * title
> * year -> *faisant référence au numéro de track*
> * numero  
>  
> forme de l'argument :
> 
> > title="titre_chanson"
> 
> Attention : le programme reconnait le titre de la chanson présent dans les métadonnées, il
> ne sagit pas du nom du fichier (il en va de même pour album/artist.. etc).  
> Le programme affichera combien de correspondance il a trouvé, puis le chemin du fichier.
> Plus d'informations sont disponibles dans src/output/request_history.md qui contient le détail
> de la dernière requète.