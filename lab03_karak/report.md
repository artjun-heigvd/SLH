# Rapport
> Voyez-vous des problèmes avec la politique spécifiée dans l’énoncé ?

1. Il n'y a pas de précision sur le fait que tout utilisateur peut changer son rôle (politique rajoutée à la fin de policiy.csv).
2. Le fait que l'admin soit tout-puissant peut poser un problème, car si le rôle est compromis ou que l'admin fait une erreur, il n'y aurait pas de vérification par un pair.
3. On ne sait pas ce qu'il se passe quand un patient ou un médecin quitte son rôle au niveau des données stockées.

> Parmi les politiques définies ci-dessus, la ou lesquelles serai(en)t pénibles à implémenter s’il fallait utiliser à la place d’ABAC un modèle RBAC traditionnel ?

Toutes politiques qui sont basées sur des relations contextuelles comme l'utilisateur créant, détruisant ou voyant son dossier personnel, ou un auteur qui accède à ses rapports...

> Que pensez-vous de l’utilisation d’un Option<UserID> mutable dans la structure Service pour garder trace de l’utilisateur loggué ? Comment pourrait-on changer le design pour savoir à la compilation si un utilisateur est censé être connecté ou pas ? Est-ce que cela premet d’éliminer une partie du traitement d’erreurs ?

Cette Option\<UserID> peut créer des erreurs quand on essaye d'accéder à l'utilisateur connecter si on ne fait pas de vérifications auparavant pendant l'exécution. 

Utiliser deux structures qui représentent différents Services (non-authentifié et authentifié) nous permettrait de vérifier à la compilation qu'une opération ne pourrait être appelé que sur un type de service définit dans les structures (on délègue aussi la vérification de l'authentification à ces structures).

> Que pensez-vous de l’utilisation de la macro de dérivation automatique pour Deserialize pour les types de model ? Et pour les types de input_validation ?
 
### `model`
L'utilisation de la dérivation automatique semble être adaptée étant donné que les types ont une correspondance proche avec leur forme sérialisée (tant que l'on veille é leur bon traitement, évidemment).

### `input_validation`

Pour ces types-là, la dérivation automatique est peu adaptée, car il n'y pas de validation des données sérialisée alors que c'est un point important de ces données.

> Que pensez-vous de l’impact de l’utilisation de Casbin sur la performance de l’application ? sur l’efficacité du système de types ?

### Performances
Malgré les optimisations mise en place par Casbin (qui le rend plus rapide qu'une implémentation "à la main"), la vérification au runtime des autorisations surcharge le traitement des demandes et peut dans un système plus conséquent poser des problèmes.

### Système de types
Avec Casbin, on perd, entre autre système à la compilation de Rust, la sûreté statique ce qui rend la détéction de problèmes plus compliquée.

> Avez-vous d’autres remarques ?

Je ne sais pas, mais plutôt non pour moins écrire.