# Étiquettes générales
not-linked = Arguments manquants
not-linked-description = Comme vous n'êtes pas lié, vous devez fournir un profil pour afficher.
showing-statistics = <a:clock:1110754973794451557> Affichage des statistiques de {$from} à {$to}.
no-previous-statistics = Aucune donnée précédente trouvée pour **{$name}**, elle a donc été insérée.
showing-guild-statistics = Affichage des statistiques de guilde de {$from} à {$to}.
no-previous-guild-statistics = Aucune donnée précédente trouvée pour la guilde **{$name}**, elle a donc été insérée.

# Errors

error-player-not-found = Un profil appartenant à {$name} n'a pas été trouvé.
error-session-not-found = Une session appartenant à {$name} n'a pas été trouvée.
error-player-uuid-not-found = Un joueur avec l'uuid {$uuid} n'a pas été trouvé.
error-player-username-not-found = Un joueur avec le nom d'utilisateur {$name} n'a pas été trouvé.
error-guild-by-member-uuid-not-found = Un membre de guilde avec l'uuid {$uuid} n'a pas été trouvé.
error-guild-by-member-username-not-found = Un membre de guilde avec le nom d'utilisateur {$name} n'a pas été trouvé.
error-guild-not-found = Une guilde avec le nom {$name} n'a pas été trouvée.
error-internal = Une erreur interne s'est produite. Il a été enregistré et sera résolu sous peu.
error-not-linked = Vous n'êtes pas lié à un compte Minecraft. Utilisez </link:1113624864524357710> pour lier votre compte.
error-invalid-uuid = L'uuid {$uuid} est invalide.
error-invalid-username = Le nom d'utilisateur {$name} est invalide.
error-member-player-not-found = Le membre {$name} n'a pas été trouvé dans le profil.
error-skyblock-profile-not-found = Un profil SkyBlock appartenant à {$name} n'a pas été trouvé.
error-player-snapshot-not-found = Aucun instantané n'a été trouvé pour {$name}. Créez-en un avec </daily bedwars:1113624864272683065>.
error-leaderboard-not-found = Un classement avec le nom {$name} n'a pas été trouvé.
error-profile-not-found = Le profil SkyBlock {$profile} appartenant à {$name} a son API désactivée ou ils ne se sont pas connectés depuis le nouveau système de profil.
error-identifier-too-long =
  Un des identifiants générés pour cette interaction est trop long.
  Essayez de réduire le nombre de composants ou la quantité de texte personnalisé.

# Quick tips

tip-background = <:knowledge_book:1117179094556233828> Changez l'arrière-plan des images générées avec </background:1117174166056075335>.
tip-history = <:knowledge_book:1117179094556233828> Affichez un graphique historique de vos statistiques avec </history bedwars:1113624864524357708>.
tip-from = <:knowledge_book:1117179094556233828> Affichez la modification des statistiques à partir d'une date spécifique avec </from bedwars:1113624864524357705>.
tip-leaderboard = <:knowledge_book:1117179094556233828> Affichez des tonnes de classements avec </leaderboard:1113624864524357709>.
tip-skyblock = <:knowledge_book:1117179094556233828> Nous prenons en charge SkyBlock ! Affichez un profil avec </skyblock profile:1113624864826327074>.
tip-link = <:knowledge_book:1117179094556233828> Liez votre compte Minecraft avec </link:1113624864524357710>.
tip-guild = <:knowledge_book:1117179094556233828> Affichez les statistiques de guilde avec </guild general:1113624864524357706>.
tip-snapshot = <:knowledge_book:1117179094556233828> Accédez à vos statistiques quotidiennes avec </daily bedwars:1113624864272683065>.
tip-display = <:knowledge_book:1117179094556233828> Changez votre format d'affichage avec </display:1113624864272683066>.
tip-help = <:knowledge_book:1117179094556233828> Obtenez plus d'aide avec </help:1113624864524357707>.
tip-website = <:knowledge_book:1117179094556233828> Visitez notre site web à l'adresse <https://statpixel.xyz>.
tip-support-discord = <:knowledge_book:1117179094556233828> Rejoignez notre serveur de support à l'adresse <https://statpixel.xyz/discord>.
tip-project = <:knowledge_book:1117179094556233828> Voir les estimations de toutes vos données avec </project bedwars:1118417616541843477>.
tip-winstreak = <:knowledge_book:1117179094556233828> Affichez vos séries de victoires avec </winstreaks:1124767485384724520>.
tip-recent = <:knowledge_book:1117179094556233828> Affichez vos dernières parties avec </recent:1123839349428080690>.
tip-bazaar = <:knowledge_book:1117179094556233828> Affichez les prix du bazar SkyBlock avec </skyblock bazaar:1113624864826327074>.

execute = execute
  .description = Exécute une commande par son identifiant
  .id = id
  .id-description = L'identifiant de la commande

invalid-identifier = Identifiant invalide fourni
invalid-identifier-description = L'identifiant fourni est invalide. Si vous avez reçu cet identifiant de StatPixel, le schéma d'identification a été mis à jour et vous devrez en obtenir un nouveau.
invalid-identifier-command-description = L'identifiant fourni n'est pas une commande.

deprecated-interaction = Interaction obsolète
deprecated-interaction-description = Cette interaction est obsolète et ne fonctionnera pas. Veuillez exécuter à nouveau la commande d'origine.
identifier = <:id:1125971775755407390> Identifiant : `{$identifier}`

# /builder

builder = builder
  .description = Crée un nouveau constructeur d'images personnalisées

builder-welcome =
  Bienvenue dans le constructeur d'images StatPixel.

  Cliquez sur les boutons ci-dessous pour ajouter votre premier composant, puis utilisez le bouton Créer pour le finaliser une fois que vous avez terminé.
  Si vous faites une erreur, utilisez Annuler pour l'annuler. Il n'y a actuellement pas de bouton Refaire, alors faites attention !

  Une fois créé, utilisez l'identifiant fourni pour afficher cette image avec vos statistiques mises à jour à tout moment, et partagez-la avec vos amis !
  Vous pouvez également l'utiliser avec notre API d'images pour l'afficher dans votre signature de forum ou n'importe où ailleurs sur Internet.

documentation = Documentation
down = Bas
down-description = Ajoute une forme directement en dessous de la forme précédente.
down-start = Bas (début)
down-start-description = Ajoute une forme en dessous de la précédente, mais tout en bas à gauche.
right = Droite
right-description = Ajoute une forme à droite de la forme précédente.
right-start = Droite (début)
right-start-description = Ajoute une forme à droite de la forme précédente, mais tout en haut.
select-position = Sélectionnez la position pour la forme

title = Titre
title-description = Affiche un nom d'utilisateur.
level-description = Affiche le niveau de n'importe quel jeu.
skin = Peau
skin-description = Une image de la peau du joueur.
bubble = Bulle
bubble-description = Une boîte pour afficher n'importe quelle statistique dans n'importe quel jeu.
subtitle = Sous-titre
subtitle-description = Un sous-titre, utilisé pour afficher un texte arbitraire jusqu'à 16 caractères.
select-shape = Sélectionnez un type de forme
select-colour = Sélectionnez une couleur

add-shape = Ajouter une forme
undo = Annuler
create = Créer

subtitle-modal-title = Créez une nouvelle forme de sous-titre
subtitle-text = Texte du sous-titre
subtitle-placeholder = Entrez jusqu'à 16 caractères

level-modal-title = Créez une nouvelle forme de niveau
level-type = Type de niveau
level-type-placeholder = Un de : bedwars, buildbattle, duels, network, pit, skywars, woolwars

bubble-modal-title = Créez une nouvelle forme de bulle
game-type = Type de jeu
game-type-placeholder = Voir la documentation : https://statpixel.xyz/docs/builder
statistic = Statistique
statistic-placeholder = Voir la documentation : https://statpixel.xyz/docs/builder

create-modal-title = Terminez la construction de votre image
username = Nom d'utilisateur
username-placeholder = Entrez le nom d'utilisateur à utiliser

invalid-statistic =
  Statistique invalide {$statistic} pour {$game}. Vérifiez la documentation et réessayez.

invalid-level-type =
  Type de niveau invalide {$kind}. Vérifiez la documentation et réessayez.

invalid-game-type =
  Type de jeu invalide {$game}. Vérifiez la documentation et réessayez.

image-created =
  Votre image a été créée ! Essayez-la avec </execute:1113624864524357708>.

  Identifiant : {$id}
  Lien : {$link}

# /about

author = Auteur 🤖
guilds = Guildes 🏰
profiles = Profils 🤺
users = Utilisateurs 🤸
snapshots = Instantanés 📒

about-description =
  StatPixel prend en charge tous les jeux du réseau Hypixel, y compris Wool Wars et SkyBlock. Pour plus d'informations, utilisez </help:1113624864524357707> ou visitez la documentation à l'adresse <https://statpixel.xyz/docs/commands>.

about = about
  .description = Affiche des informations sur le bot

# /winstreaks

winstreaks = winstreaks
  .description = Affiche les séries de victoires
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = Minecraft UUID

# /recent

mode = Mode
map = Carte
started = Début
duration = Durée
playing = Actif

recent = recent
  .description = Affiche les dernières parties d'un joueur
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = Minecraft UUID

# /project

accuracy = Précision
estimate = Estimation
never = Jamais

# /background

background = background
  .description = Change l'arrière-plan des images générées
  .colour = couleur
  .colour-description = La couleur de l'arrière-plan

error-invalid-colour = Couleur invalide fournie
error-invalid-colour-description = La couleur fournie est invalide. Essayez de fournir l'une de [ces couleurs](https://simple.wikipedia.org/wiki/List_of_colors) ou une couleur hexadécimale préfixée par `#`.
colour-changed = Couleur de fond modifiée
colour-changed-description = Votre couleur de fond a été modifiée en {$colour}.

# /skyblock bazaar

buy-price = Prix d'achat
sell-price = Prix de vente
last-hour = Dernière heure
last-day = Dernier jour
last-week = Dernière semaine

bazaar = bazaar
  .description = Affiche les prix du bazar SkyBlock
  .product = produit
  .product-description = Le produit à afficher

# /skyblock auctions

Auctions = Enchères
Bank = Banque
Candy = Bonbons
EnderChest = Coffre de fin
Equipment = Équipement
Fishing = Pêche
Inventory = Inventaire
Networth = Valeur nette
Pets = Animaux de compagnie
Potions = Potions
Profile = Profil
Quiver = Carquois
Talisman = Talisman
Vault = Coffre-fort
Wardrobe = Garde-robe

auctions = auctions
  .description = Affiche les enchères SkyBlock.
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft

player-auctions = Enchères
highest-bid = Offre la plus élevée

# /skyblock inventory

inventory = inventory
  .description = Affiche un inventaire SkyBlock.
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

enderchest = enderchest
  .description = Affiche un coffre de fin SkyBlock.
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

quiver = quiver
  .description = Affiche un carquois SkyBlock.
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

talisman = talisman
  .description = Affiche un sac de talismans SkyBlock.
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

fishing = fishing
  .description = Affiche un sac de pêche SkyBlock
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

potions = potions
  .description = Affiche un sac de potions SkyBlock
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

equipment = equipment
  .description = Affiche l'équipement SkyBlock
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

wardrobe = wardrobe
  .description = Affiche une garde-robe SkyBlock
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

candy = candy
  .description = Affiche un inventaire de bonbons SkyBlock
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

vault = vault
  .description = Affiche un coffre personnel SkyBlock
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

pets = pets
  .description = Affiche les animaux SkyBlock
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

networth = networth
  .description = Affiche la valeur nette du profil SkyBlock
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

# /skyblock profile

profile = profile
  .description = Affiche un profil SkyBlock.
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

member-profile = Profil de membre
fairy-souls = Âmes de fées
fairy-exchanges = Échanges
fishing-treasure = Trésor
zones-visited = Zones
generators-crafted = Générateurs
highest-crit = Crit le plus élevé

farming = Agriculture
mining = Minage
combat = Combat
foraging = Récolte
fishing-skill = Pêche
enchanting = Enchantement
alchemy = Alchimie
taming = Dressage
dungeoneering = Donjon
carpentry = Menuiserie
runecrafting = Runecraft
social = Social

# /skyblock bank

bank = bank
  .description = Affiche une banque SkyBlock.
  .username = username
  .username-description = Nom d'utilisateur Minecraft
  .uuid = uuid
  .uuid-description = UUID Minecraft
  .profile = profile
  .profile-description = Le profil à afficher

island-bank-balance = Solde de la banque de l'île
bank-balance = Solde de la banque

# /leaderboard

leaderboard = leaderboard
  .description = Affiche le classement de différents jeux.
  .board = board
  .board-description = Le classement à afficher

# /network

network = network
  .description = Affiche les statistiques du réseau d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .hours = hours
  .hours-description = Le nombre d'heures à afficher
  .days = days
  .days-description = Le nombre de jours à afficher
  .weeks = weeks
  .weeks-description = Le nombre de semaines à afficher

karma = Karma
rewards = Récompenses
friend-requests = Demandes d'amis
first-login = Première connexion
last-login = Dernière connexion
quests = Quêtes
challenges = Défis
achievement-points = Points de succès
language = Langue
gifts-given = Cadeaux donnés
ranks-given = Rangs donnés

# /help

help = help
  .description = Affiche le menu d'aide.

help-general = Général <:cookie:1115091335565811775>
help-general-description = StatPixel prend en charge tous les jeux du réseau Hypixel. Vous pouvez afficher les statistiques de chaque jeu en utilisant `/<jeu>`. Par exemple, essayez </bedwars:1113624864272683060> !

help-display = Affichage <:spyglass:1115091333657411625>
help-display-description = Si vous êtes sur une connexion mesurée ou si vous voulez simplement économiser des données, vous pouvez recevoir des réponses dans différents formats, comme du texte ou des images condensées, avec </display:1113624864524357705>.

help-link = Liaison <a:recovery_compass:1115091332680126504>
help-link-description = Taper votre nom d'utilisateur pour chaque commande peut devenir fastidieux. Pour rendre cela plus facile, vous pouvez lier un compte sans vérification avec </link:1113624864524357710> et le dissocier plus tard avec </unlink:1113624865262538854>.

help-snapshot = Instantanés <:book_and_quill:1110754805724479569>
help-snapshot-description = Les instantanés sont un moyen de visualiser les changements de vos statistiques au fil du temps. Il existe quelques commandes utilitaires pour vous faciliter la vie : </daily bedwars:1113624864272683065>, </weekly bedwars:1113624865262538858> et </monthly bedwars:1113624864524357712>. Celles-ci fonctionnent également pour votre guilde, alors essayez </daily guild:1113624864272683065> !
  
  Si vous voulez afficher une plage plus spécifique, essayez </from bedwars:1113624864524357705>.

help-history = Historique <a:clock:1115091329958019253>
help-history-description = Pour afficher un graphique de l'évolution de statistiques spécifiques au fil du temps, essayez </history network:1113624864524357708>.

help-image-builder = Image Builder 🖌️
help-image-builder-description = Créez vos propres images dynamiques avec l'Image Builder ! Utilisez </builder:1113624864524357709> pour commencer ou allez sur https://statpixel.xyz/docs/builder pour plus d'informations.

# /history

statistics-history = {" "}historique pour{" "}

# /display

display = display
  .description = Modifie la façon dont les réponses sont affichées.
  .format = format
  .format-description = Le format de réponse à utiliser

display-changed = Affichage modifié
display-changed-text-description = Les réponses seront désormais envoyées sous forme de texte.
display-changed-image-description = Les réponses seront désormais envoyées sous forme d'images lorsque cela est applicable.
display-changed-compact-description = Les réponses seront désormais envoyées sous forme d'images compactes lorsque cela est applicable.

Image = Image
Compact = Compact
Text = Text

# /unlink
unlink = unlink
  .description = Dissocie votre compte Discord d'un compte Minecraft.

unlinking-failed = Suppression du lien échouée
unlinking-failed-description = Vous n'êtes pas lié à un compte Minecraft.
unlinking-succeeded = Lien supprimé avec succès
unlinking-succeeded-description = Vous n'êtes plus lié à un compte Minecraft.

# /link
link = link
  .description = Lie votre compte Discord à un compte Minecraft.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à lier
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à lier

linking-failed = Échec de la liaison
linking-failed-uuid-description = L'UUID « {$uuid} » n'appartient pas à un compte Minecraft.
linking-failed-username-description = Le nom d'utilisateur **{$username}** n'appartient pas à un compte Minecraft.
linking-failed-description = Vous devez fournir un UUID ou un nom d'utilisateur valide.
linking-succeeded = Liaison réussie
linking-succeeded-description = Votre compte Discord est désormais lié au compte Minecraft **{$name}**.

# /arcade

Party = Fête
SantaSays = Père Noël dit
SimonSays = Simon dit
MiniWalls = Mini murs
Soccer = Football
OneInTheQuiver = Un dans la fronde
EnderSpleef = Ender Spleef
FarmHunt = Chasse à la ferme
DragonWars = Guerres des dragons
BlockingDead = Blocs morts
Zombies = Zombies
ZombiesBadBlood = Zombies : Mauvais sang
ZombiesDeadEnd = Zombies : Impasse
PixelPainters = Peintres de pixels
HoleInTheWall = Trou dans le mur
ThrowOut = Jeter
EasterSimulator = Simulateur de Pâques
ScubaSimulator = Simulateur de plongée
HalloweenSimulator = Simulateur d'Halloween
GrinchSimulator = Simulateur de Grinch

mystery-gifts = Cadeaux mystères

arcade = arcade
  .description = Affiche les statistiques d'Arcade d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Arcade à afficher

# /arena

magical-keys = Clés magiques
magical-chests = Coffres magiques
rating = Classement

arena = arena
  .description = Affiche les statistiques du combat d'arène d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher.
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher.
  .mode = mode
  .mode-description = Le mode de combat d'arène à afficher.

# /bedwars

Solo = Solo
Double = Doubles
Three = Trios
Four = Quatruples
SoloRush = Rush Solo
DoubleRush = Rush Doubles
FourRush = Rush Quatruples
SoloUltimate = Ultimate Solo
DoubleUltimate = Ultimate Doubles
FourUltimate = Ultimate Quatruples
Castle = Château
DoubleLucky = Lucky Doubles
FourLucky = Lucky Quatruples
DoubleVoidless = Voidless Doubles
FourVoidless = Voidless Quatruples
DoubleArmed = Armed Doubles
FourArmed = Armed Quatruples
DoubleUnderworld = Underworld Doubles
FourUnderworld = Underworld Quatruples
DoubleSwap = Swap Doubles
FourSwap = Swap Quatruples

final-kills = Victimes finales
final-deaths = Morts finales
fkdr = VKTR
beds-broken = Lits brisés
beds-lost = Lits perdus
bblr = LBPL

iron-collected = Fer
gold-collected = Or
diamond-collected = Diamants
emerald-collected = Emeraudes
items-purchased = Achats

bedwars = bedwars
  .description = Affiche les statistiques du combat de lit d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher.
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher.
  .mode = mode
  .mode-description = Le mode de combat de lit à afficher.

# /blitz

Armorer = Armurier
Scout = Éclaireur
Speleologist = Spéléologue
Random = Aléatoire
Rogue = Voleur
Rambo = Rambo
Troll = Troll
HorseTamer = Dresseur de chevaux
WolfTamer = Dresseur de loups
Warrior = Guerrier
Phoenix = Phoenix
DonkeyTamer = Dresseur d'ânes
Ranger = Ranger
Archer = Archer
Necromancer = Nécromancien
Meatmaster = Maître des viandes
Tim = Tim
Pigman = Homme-porc
CreeperTamer = Dresseur de creepers
Florist = Fleuriste
Warlock = Ensorceleur
Milkman = Livreur de lait
Astronaut = Astronaute
Blaze = Blaze

potions-drunk = Potions bues
chests-opened = Coffres ouverts
time-played = Temps de jeu

blitz = blitz
  .description = Affiche les statistiques des jeux de survie Blitz d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher.
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher.
  .mode = mode
  .mode-description = Le mode de jeu de survie Blitz à afficher.

# /buildbattle

SoloPro = Solo Pro
GuessTheBuild = Devine le Build

votes = Votes
most-points-solo = Plus de points (Solo)
most-points-team = Plus de points (Équipe)

buildbattle = buildbattle
  .description = Affiche les statistiques de Build Battle d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Build Battle à afficher

# /copsandcrims

Defusal = Désamorçage
GunGame = Gun Game
Deathmatch = Deathmatch

cop-kills = Meurtres de policier
criminal-kills = Meurtres de criminel
headshot-kills = Meurtres en visant la tête
grenade-kills = Meurtres à la grenade
bombs-defused = Bombes désamorcées
bombs-planted = Bombes posées

copsandcrims = copsandcrims
  .description = Affiche les statistiques de Cops and Crims d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Cops and Crims à afficher

# /duels

UhcSolo = Duel UHC Solo
OpSolo = Duel OP Solo
UhcDouble = Duel UHC Doubles
BowSolo = Duel à l'arc Solo
ClassicSolo = Duel classique Solo
OpDouble = Duel OP Doubles
UhcFour = UHC 4v4
SkyWarsDouble = Duel Sky Wars Doubles
SumoSolo = Duel Sumo Solo
SkyWarsSolo = Duel Sky Wars Solo
BridgeDoubleDuel = Bridge 2v2
BridgeFourDuel = Bridge 4v4
BridgeSolo = Duel Bridge
BridgeThree = Bridge à trois
BridgeDouble = Bridge Doubles
ComboSolo = Duel Combo Solo
SumoTournament = Tournoi Sumo
SkyWarsTournament = Tournoi Sky Wars
UhcMeetup = UHC Meetup
PotionSolo = Duel Potion Solo
BlitzSolo = Duel Blitz Solo
BowSpleefSolo = Duel Bow Spleef Solo
MegaWallsSolo = Duel Mega Walls Solo
BoxingSolo = Duel de boxe Solo
Parkour = Parkour
ArenaSolo = Duel d'arène Solo
CaptureThree = Capture à trois
BridgeThreeDuel = Bridge 3v3

melee-accuracy = Précision au corps-à-corps
health-regenerated = Santé régénérée

duels = duels
  .description = Affiche les statistiques de Duels d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Duels à afficher

# /megawalls

FaceOff = Face Off

distance-walked = Distance parcourue
distance-fallen = Distance de chute
bread-eaten = Pain mangé
wood-chopped = Bois coupé
treasures-found = Trésors trouvés

megawalls = megawalls
  .description = Affiche les statistiques Mega Walls d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Mega Walls à afficher

# /murdermystery

Assassins = Assassins
Classic = Classique
DoubleUp = Double
Infection = Infection

time-survived = Temps Survécu
murderer-wins = Victoires du meurtrier
detective-wins = Victoires du détective

murdermystery = murdermystery
  .description = Affiche les statistiques Murder Mystery d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Murder Mystery à afficher

# /paintball

adrenaline = Adrénaline
endurance = Endurance
fortune = Fortune
godfather = Parrain
superluck = Superchance
transfusion = Transfusion
kill-prefix = Préfixe de mort
show-kill-prefix = Afficher le préfixe de mort

shots-fired = Tirs effectués
killstreaks = Séries de meurtres
forcefield-time = Temps de force de champ
chat-messages = Messages de chat
soups-drank = Soupes bues
cash-earned = Argent gagné
highest-killstreak = Meilleure séquence

paintball = paintball
  .description = Affiche les statistiques Paintball d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Paintball à afficher

# /pit

cash = Argent
bow-damage-dealt = Dégâts d'arc infligés
bow-damage-taken = Dégâts d'arc subis
bdr = BDR
contracts-completed = Contrats terminés
contracts-started = Contrats commencés
cr = Taux de complétion

pit = pit
  .description = Affiche les statistiques The Pit d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode The Pit à afficher

# /quake

SoloTournament = Tournoi Solo

hr = Taux de Headshot
headshots = Headshots
sight = Visée

quake = quake
  .description = Affiche les statistiques de Quakecraft d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Quakecraft à afficher

# /skywars
Overall = Général
SoloNormal = Solo Normal
SoloInsane = Solo Insane
TeamNormal = Équipe Normal
TeamInsane = Équipe Insane
MegaNormal = Mega Normal
MegaDouble = Mega Doubles
Ranked = Classé
SoloLab = Solo Lab
TeamLab = Équipe Lab
Tourney = Tournoi

opals = Opales
heads = Têtes
souls = Âmes
tokens = Jetons
bow-accuracy = Précision à l'arc
eggs-thrown = Oeufs lancés
fastest-win = Victoire la plus rapide

skywars = skywars
  .description = Affiche les statistiques de Sky Wars d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Sky Wars à afficher

# /smash

smasher = Briseur
smashed = Brisé
ssr = SSR

smash = smash
  .description = Affiche les statistiques de Smash Heroes d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Smash Heroes à afficher

# /speeduhc

tears = Larmes
survived-players = Joueurs survivants

speeduhc = speeduhc
  .description = Affiche les statistiques de SpeedUHC d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode SpeedUHC à afficher

# /tntgames

TntRun = TNT Run
TntTag = TNT Tag
PvpRun = PvP Run
BowSpleef = Bow Spleef
Wizard = Magiciens

record = Record
double-jumps = Doubles sauts
tags = Étiquettes
air-time = Temps d'air
points = Points

tntgames = tntgames
  .description = Affiche les statistiques de TNT Games d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode TNT Games à afficher

# /turbokartracers

box-pickups = Ramassage de boîtes
coin-pickups = Ramassage de pièces
grand-prix = Grand Prix
show-prefix = Afficher le préfixe
bronze-trophies = Trophées de bronze
silver-trophies = Trophées d'argent
gold-trophies = Trophées d'or

turbokartracers = turbokartracers
  .description = Affiche les statistiques de Turbo Kart Racers d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Turbo Kart Racers à afficher

# /uhc

RedVsBlue = Rouge contre Bleu
NoDiamonds = Pas de diamants
VanillaDouble = Double vanille
Brawl = Bagarre
SoloBrawl = Bagarre solo
DoubleBrawl = Bagarre en duo

heads-eaten = Têtes mangées
ultimates-crafted = Ultimates créées

uhc = uhc
  .description = Affiche les statistiques d'UHC Champions d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode UHC Champions à afficher

# /vampirez

human-wins = Victoires humaines
vampire-wins = Victoires de vampires
zombie-kills = Zombies tués
human-kills = Humains tués
human-deaths = Morts humaines
vampire-kills = Vampires tués
vampire-deaths = Morts de vampires
blood = Sang
starting-compass = Boussole de départ
starting-gear = Équipement de départ
tracker = Suiveur
updated = Mis à jour
old-vampire = Ancien vampire
hkdr = HKDR
vkdr = VKDR

vampirez = vampirez
  .description = Affiche les statistiques de VampireZ d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode VampireZ à afficher

# /walls

Standard = Standard

activations = Activations
iron-broken = Fer cassé

walls = walls
  .description = Affiche les statistiques de The Walls d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode The Walls à afficher

# /warlords

CaptureTheFlag = Capture du drapeau
Domination = Domination
TeamDeathmatch = Match à mort par équipe

wins-blue = Victoires (Bleu)
wins-red = Victoires (Rouge)
hide-prestige = Masquer
mvps = MVPs

warlords = warlords
  .description = Affiche les statistiques de Warlords d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Warlords à afficher

# /woolwars

layers = Layers
powerups-collected = Powerups Collectés
wool-placed = Laine Posée

woolwars = woolwars
  .description = Affiche les statistiques Wool Wars d'un joueur.
  .username = username
  .username-description = Le nom d'utilisateur Minecraft à afficher
  .uuid = uuid
  .uuid-description = L'UUID Minecraft à afficher
  .mode = mode
  .mode-description = Le mode Wool Wars à afficher

# /guild

daily-xp = XP Quotidien
weekly-xp = XP Hebdomadaire
monthly-xp = XP Mensuel
xp-since = XP Depuis
members_label = Membres
date = Date
weekly-gexp = GEXP Hebdomadaire
position = Position
guild-quests = Quêtes de guilde

Member = Membre
General = Général
Members = Membres
Top = Top

member = member
  .description = Affiche le membre d'une guilde.
  .username = username
  .username-description = Le nom d'utilisateur de la guilde du membre à afficher.
  .uuid = uuid
  .uuid-description = L'UUID de la guilde du membre à afficher.

guild = guild
  .description = Affiche les statistiques d'une guilde.
  .name = name
  .name-description = Le nom de la guilde à afficher
  .username = username
  .username-description = Le nom d'utilisateur de la guilde du membre de la guilde à afficher.
  .uuid = uuid
  .uuid-description = L'UUID de la guilde du membre de la guilde à afficher.

general = general
  .description = Affiche les statistiques d'une guilde.
  .name = name
  .name-description = Le nom de la guilde à afficher
  .username = username
  .username-description = Le nom d'utilisateur de la guilde du membre de la guilde à afficher.
  .uuid = uuid
  .uuid-description = L'UUID de la guilde du membre de la guilde à afficher.

members = members
  .description = Affiche les membres d'une guilde.
  .name = name
  .name-description = Le nom de la guilde à afficher
  .username = username
  .username-description = Le nom d'utilisateur de la guilde du membre de la guilde à afficher.
  .uuid = uuid
  .uuid-description = L'UUID de la guilde du membre de la guilde à afficher.

top = top
  .description = Affiche les membres d'une guilde par XP.
  .name = name
  .name-description = Le nom de la guilde à afficher
  .username = username
  .username-description = Le nom d'utilisateur de la guilde du membre de la guilde à afficher.
  .uuid = uuid
  .uuid-description = L'UUID de la guilde du membre de la guilde à afficher.
  .days = days
  .days-description = jours
  .limit = limit
  .limit-description = Le nombre de membres à afficher

showing-guild-xp-statistics = Affiche les gains d'XP de la guilde de {$from} à {$to}.

# Étiquettes partagées

Normal = Normal
Team = Équipe

blocks-broken = Blocs cassés
blocks-placed = Blocs posés

coins = Pièces
loot-chests = Coffres au butin

offline = Hors ligne
online = En ligne
level = Niveau
progress = Progrès

wins = Victoires
losses = Défaites
wlr = TDR
win-streak = Série de victoires
kills = Éliminations
deaths = Morts
kdr = TDK
assists = Assistances
games-played = Parties jouées
wr = Taux de victoire
damage-dealt = Dégâts infligés
damage-taken = Dégâts subis
ddtr = TDRD
games = Parties
score = Score
created-at = Créé le
experience = Expérience

yes = Oui
no = Non
none = Rien

# Couleurs

black = Noir
dark-blue = Bleu foncé
dark-green = Vert foncé
dark-aqua = Aqua foncé
dark-red = Rouge foncé
dark-purple = Violet foncé
gold = Or
gray = Gris
dark-gray = Gris foncé
blue = Bleu
green = Vert
aqua = Aqua
red = Rouge
light-purple = Violet clair
yellow = Jaune
white = Blanc
