# Réponses questions
### Q1
> Lesquelles des affirmations suivantes sont vraies ou fausses ? (justifier chaque réponse).
> - L’Autorité peut accéder à toutes les informations médicales si elle le désire

Vrai, car c'est le référent unique pour l'autorisation, elle peut donc se générer les biscuits dont elle a besoin.
> - L’Autorité peut déterminer avec quels médecins un patient partage ses données

Faux, car on va passer par des biscuits atténués pour donner l'accès aux médecins, le référent unique (donc l'autorité) n'aura pas accès à ces informations du fait que l'on a pas besoin de passer par lui pour ces biscuits atténués.
> - Un fournisseur peut déterminer tous les médecins avec lesquels un patient partage ses données

Vrai, car il pourra voir quels médecins a accès à quelles données du patient, il peut donc en déduire avec quels médecins le patient partage ses données.
> - Un médecin peut partager des données auxquelles il a accès avec autre médecin, sans le consentement du patient.

Faux, car la politique du biscuit atténué devrait empêcher cela en ajoutant les checks nécessaires.
### Q2
> En principe, les utilisateurs doivent pouvoir s’inscrire librement auprès de l’autorité, mais les médecins doivent être validés manuellement. Le développeur a toutefois laissé une backdoor pour l’enregistrement des médecins. Quelle URL pouvez-vous utiliser pour enregistrer un médecin ? Sous quelle CWE pourrait-t-on classer cette vulnérabilité ?
 

### Q3
> Examinez le fonctionnement du processus de login entre l’exécutable client (src/bin/karamel.rs) et le serveur d’autorité; que contient le message envoyé par le serveur au client, que contient la réponse en cas de succès, et que fait le client avec la réponse ?

Le serveur va lui envoyer un token si le login réussi que le client va écrire dans `.karamel-token`. Ce token est le biscuit qui donne l'identifiant unique du user et s'il est un docteur. En cas d'échec de login alors le serveur lui répondra par un message d'erreur `Status::Forbidden`.
### Q4
>Pour assurer la confidentialité des mots de passe pendant le processus de login, quelle fonctionnalité essentielle devrait absolument être ajoutée avant de déployer ce code en production ?

Il faudrait absolument hasher les mots de passe avant de les envoyer à l'autorité afin de les protéger (on compare ensuite les hashs).
### Q5
>Examinez le fonctionnement du serveur de stockage. Comment la clé publique de l’autorité est-elle passée au processus du serveur de stockage ?

Par un fichier `pubkey.bin` que l'on lit au démarrage du serveur de stockage.
### Q6
> read_report dans bin/store.rs renvoie une erreur 404 si l’ID du rapport demandé n’existe pas, avant de procéder à la décision d’autorisation. Est ce un problème ? Justifier.

On peut en déduire les ids des rapports qui existent et ceux qui n'existent pas, cela pourrait éventuellement poser problème si on recherce un rapport avec un ID en particulier pour pouvoir vérifier sans autorisation s'il existe ou non. Donc c'est plutôt un problème.
### Q7
>


### Q8
> Listez les noms des faits datalog disponibles pour effectuer l’autorisation, en indiquant ceux provenant du biscuit et ceux provenant du contexte de la requête

Il y a plusieurs faits pour l'autorisation sur :
- `user` : l'id de l'utilisateur qui provient du biscuit
- `is_doctor` : si l'utilisateur est un docteur ou non qui provient du biscuit
- `id` : l'id du rapport qui provient du contexte
- `author` : l'auteur du rapport qui provient du contexte
- `patient` dans `add_report_facts`: le patient concérné par le rapport qui provient du contexte
- `report_time` : la date du rapport qui provient du contexte
- `keyword` : les mots clés dans le rapport qui proviennent du contexte
- `patient` dans `add_patient_facts` : l'id du patient qui provient du contexte
- `blood_type` : le groupe sanguin du patient qui provient du contexte
- `gender` : le genre du patient qui provient du contexte
### Q9
> Par rapport à un système d’ABAC tel que vu dans Karak™, y a-t-il des limitations sur la manière dont les règles d’accès peuvent dépendre du contenu d’un rapport ?

Oui, car on ne peut pas faire de règles sur le contenu que dont on a pas créé des faits, ce qui limite le nombre de règles en général, car il serait difficile de faire des faits pour tout le contenu d'un rapport.
### Q11
>


### Q12
> Vous désirez fournir à votre médecin l’autorisation de lire certains rapports médicaux. Proposez une commande pour créer un token attenué permettant l’accès à chacun de ces cas:
> - un seul rapport en particulier

token attenué : `token.append(block!(r#"check if id({report_id})"#))`
Il nous reste juste à donner le report_id que l'on veut.
> - tous les rapports concernant les problèmes de coeur (keyword « heart »), et postérieurs au 1er janvier 2010

token attenué :
```rust
let mut builder = block!(r#"check if keyword(heart)"#);
builder.append(block!(r#"check if report_time($time), $time > 2010-01-01"#));
token.append(builder);
```
> - tous les rapports par un médecin particulier

token attenué : `token.append(block!(r#"check if author({doc_id})"#))`
On donne l'id du docteur.