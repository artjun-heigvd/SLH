# Réponses questions
Arthur Junod

---
> Les passkeys, comme l’authentification unique (SSO), permettent toutes deux à un utilisateur d’utiliser un seul mécanisme de sécurité pour tous ses services, sans s’exposer à une attaque de "credentials stuffing". Mais quelle est la différence en termes de risque ?

Avec un SSO, on utilise un seul mot de passe pour sécuriser plusieurs plateformes, mais on reste donc sur l'utilisation d'un mot de passe qui est sensible aux attaques, car il n'est pas stocké dans un composant dédié et peut-être également mémorisé ou intercépté.

Avec l'utilisation des passkeys, on va la stocker dans des composants dédiés, la passkey privée ne s'expose jamais en dehors des appareils de l'utilisateur et ne peut pas être intercepté et finalement, elle ne peut techniquement pas être mémorisée par un humain.

Ces différences permettent aux passkeys de ne pas se faire intercepter, de ne pas être sensible aux attaques de social engineering, car l'utilisateur lui-même ne connait pas sa private passkey et d'être pus difficilement accessible même en ayant accès d'une quelconque manière à l'appareil de l'utilisateur.

> Concernant la validation d’entrées pour les images, quelles sont les étapes que vous avez effectuées ? Et comment stockez vous les images ?

Pour valider les entrées pour les images, j'ai effectué les étapes suivantes :
1. Validation des e-mails : Intégration de UserEmail::try_new() pour valider les adresses e-mail selon les spécifications HTML5 dans les handlers d'enregistrement et d'authentification, garantissant ainsi que seules les adresses bien formées sont traitées.
2. Validation du contenu texte : Utilisation de TextualContent pour vérifier les entrées textuelles, en s'assurant qu'elles sont exemptes de caractères de contrôle et de HTML, et respectent les contraintes de longueur pour les courtes et longues formes, notamment dans la création de posts.
3. Validation des images : Utilisation de validate_image() pour garantir que les images téléchargées sont des fichiers JPEG valides, en vérifiant à la fois le format et l'extension du fichier, et en générant des noms de fichiers uniques pour éviter les attaques de chemin d'accès et les collisions.
4. Amélioration de la sécurité : Grâce à la validation des entrées, atténuation des risques tels que les attaques par injection (par exemple, XSS) et garantie de l'intégrité des données avant stockage ou traitement ultérieur.

En intégrant ces validations, nous avons renforcé l'application contre les vulnérabilités courantes des entrées, améliorant ainsi la sécurité et la fiabilité des données.

> Que pensez-vous de la politique de choix des noms de fichiers uploadés ? Y voyez-vous une vulnérabilité ? Si oui, suggérez une correction.

La politique de choix des noms de fichiers uploadés peut présenter une vulnérabilité si les noms de fichiers ne sont pas correctement gérés. Par exemple, si les noms de fichiers sont directement dérivés des noms fournis par les utilisateurs, cela peut entraîner des conflits de noms ou des attaques par téléchargement de fichiers malveillants.

Pour corriger cela, il est recommandé de générer des noms de fichiers uniques pour chaque fichier uploadé. Cela peut être fait en utilisant un identifiant unique (par exemple, un UUID) ou en utilisant un hachage du contenu du fichier. De plus, il est important de vérifier et de nettoyer les noms de fichiers pour éviter l'injection de caractères spéciaux ou de chemins de fichiers.
 
