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
> Il vous sera ensuite demandé si vous souhaitez conserver le résultat de cette requête,
> si tel est le cas, les informations de votre dernière requête se trouveront dans le fichier 
> *"src/output/request_history.md"*.
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
> Plus d'informations sont disponibles dans *"src/output/request_history.md"* qui contient le détail
> de la dernière requète (si vous avez choisi l'option write).

#### Utilisation avec arguments :
> La syntaxe est la suivante et varie en fonction de la première commande choisie :
>
> > cargo run command arg2 arg3 arg4
> 
> **command** correspond aux 3 tâches disponibles dans le programme :  
> * scan
> * search
> * playlist
>  
> **arg2** varie en fonction de la 1ʳᵉ commande utilisée :
> * scan src/test . . . .*un path dans le cas de scan*
> * search title="titre_chanson" . . . .*une contrainte dans le cas de search*
> * playlist src/test . . . .*un path dans le cas de playlist*  
>  
> **arg3** suis le même fonctionnement :
> * scan src/test w . . . .*(optionnel) choix d'écrire le résultat de la requête*
> * search title="titre_chanson" . . . .*(optionnel) choix d'écrire le résultat de la requête*
> * playlist src/test title="titre_chanson" . . . .*(obligatoire) contrainte pour la sélection dans la playlist*  
>   
> **arg4** utilisé uniquement dans le cas de playlist :
> * playlist src/test title="titre_chanson" w . . . .*(optionnel) choix d'écrire le résultat de la requête*

#### Rapide tour des modules :
> > **Cli** contient une structure permettant de faciliter la lecture des argumes.
> 
> > **musicfile** contient les structure musicfiles et MFcontainer (qui les englobes) après un scan, les données des
> fichiers trouvés (path/metadata ..) seront stockés dans des instances de musicfiles.
> 
> > **scan** module utilisé pour parcourir les dossiers depuis la racine donnée, en vérifiant la capacité de chaques
> fichiers d'être supportés par le programme. Si tel est le cas, le fichier est retenu et enregistré dans une instance
> de musicfile. Ils sont ensuite tous stockés dans une instance de MFcontainer.  
> Enfin, le module va serializer et écrire dans un json le MFcontainer afin de stocker en mémoire le résultat.
> 
> > **search** module qui une fois un scan fait et selon des contraintes passés en argument, va sélectionner parmis les
> fichiers retenus (donc dans le json créer par scan) ceux présentant les bonnes métadonnées (artiste/album/titre..).
> 
> > **playlist** module qui va s'appuyer sur les deux précédents. Playlist va lancer un scan depuis la racine donnée,
> puis effectuer un search selon les contraintes données. Une fois le procéssus terminés, les fichiers ayant les
> bonnes spécifications sont convertis en playlist compatible avec vlc (fichier trouvable dans src/output).
> 
> > **interface** module lancé lorsque le programme ne détecte aucun argument, c'est une simple ihm question/réponse
> qui est ammené a lancer une des 3 requète disponible.