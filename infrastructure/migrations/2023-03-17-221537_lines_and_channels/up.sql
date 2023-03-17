-- Your SQL goes here
CREATE TABLE "channels" (
	"id"	INTEGER NOT NULL,
	"alias_id"	INTEGER NOT NULL,
	"is_parted"	INTEGER NOT NULL DEFAULT 0,
	"joined_at"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);

CREATE TABLE "lines" (
	"id"	INTEGER NOT NULL,
	"line"	TEXT NOT NULL,
	"category_id" INTEGER NOT NULL DEFAULT 1,
	"channel_id"	INTEGER,
	"is_disabled"	INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("channel_id") REFERENCES "channels"("id")
);

INSERT INTO lines (line, category_id) VALUES
("me: drinking milk... someone: it's not milk... me: drinking NOT milk... someone: it's my... me: *quickened up...* 🥛 🥛 🥛 😋 ", 3),
("uhmmm... it's kind of my milk... do you like it? 🥛 😳 ", 3),
("i'm literally 'not milk' 🥛 Emotionless ", 3),
("average 'not milk' enjoyer 🥛 GIGACHAD ", 3),
("we are not 'born to die', does a cup of 'not milk' poured to finish immediately? 🥛 FeelsStrongMan ", 3),
("the cold cruelty of the universe when the indomitable 'not milk' spirit walks into the room: 🥛 WAITWAITWAIT ", 3),
("you drank the milk and it tastes like the milk from your childhood 🥛 👩 ", 2),
("your lunch is here, commander... oh no, i forgot the milk! don't worry 🥛 🤨 ", 2),
("for every kill you get i'll give you a 'not milk' ... Alright bet 🥛 😳 ", 2),
("i may not show it but a cup of 'not milk' really be giving me renewed purpose in life 🥛 💯 ", 2),
("'why should i even keep drinking??' mfs when they look back at where they started to drink 🥛 🥹", 2),
("when i am in a 'restoring faith in humanity' competition and my opponent is 'not milk' 🥛 OMAYGOT ", 2),
("you are completely clueless of the contents of your drink 🥛 Clueless ", 1),
("linus milk tips 🥛 🤓 ", 1),
("bitches be like 'i am poly'... ok, do you want a 'not milk'? 🥛 💀 ", 1),
("he has 97 mental illnesses and is banned from most public spaces but he's your milk producer 🥛 BrorStirrer ", 1),
("'your gender is what's in your pants' 🤓  alright, my gender is 'milk' 🥛 🔥 ", 1),
("be the reason why someone continues to produce 'not milk' as the main purpose of their life", 1),
("it's alright - when i'm wearing a skirt... i'm a 'not milk' consumer 🥛 😳 👉 👈 ", 1),
("you really got milk. real milk 🥛 😔 ", 0),
("haha!!! you just drank SULFURIC ACID!!! 🥛 ☠ ", 0),
("average milk fan 🥛 🐄 🤓 ", 0);
