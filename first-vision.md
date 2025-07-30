# The First Vision: The Text Adventure

@start starting_room

## @room starting_room
My name is Joseph Smith. I was born in the year of our Lord one thousand eight hundred and five, on the twenty-third day of December, in the town of Sharon, Windsor county, State of Vermont. My father, Joseph Smith, Sen., left the State of Vermont, and moved to Palmyra. We were, by necessity, required to obtain a scanty maintenance through our daily labor. I am at the farm with my brothers, working to gather wheat into large bales.

### Bale some hay [hay_bales < 3]
- After considerable effort, I have made another bale of hay.
- count+ hay_bales

### Ask Alvin about the work [hay_bales = 0]
- "Alvin says, "Hey, Joseph! Looking strong today! Do you think you could do at least three bales of hay?"

### Talk to Alvin about tomorrow [hay_bales = 1]
- Alvin says, "I'm looking forward to a day of rest! What church do you think you'll go to this week?"

### Talk to Alvin about churches [hay_bales = 2]
- Alvin says, "I was thinking about all the different preachers. I don't know if any of those preachers know what they are talking about."

### Talk to Alvin [hay_bales > 2]
- Alvin says, "Good job, Joseph! I know it's hard work. I'm proud of you. I'm happy to be your brother."

### Talk to Hyrum about the work [hay_bales = 0]
- "Hyrum says, "Don't worry, Joseph, I know you still struggle with your limp sometimes. I can bale hay for us if you are feeling tired.""

### Talk to Hyrum [hay_bales = 1]
- Hyrum says, "This is hard work, but I wouldn't rather be anywhere else. I love working with my brothers."

### Talk to Hyrum about Alvin [hay_bales = 2]
- Hyrum says, "Alvin always clenches his jaw when he is angry. I saw his jaw clenched last Sunday when the preacher said that unbaptized children go to hell."

### Talk to Hyrum [hay_bales > 2]
- Hyrum says "Good job, Joseph!"

### Go home [hay_bales > 2]
- @home

## @room home
After a hard day of work, it is nice to be inside. The daylight grows dim, and we light up our lanterns. Our family is gathered around the fireplace to listen to the scriptures, as we do each night before bed. Father pulls out his reading glasses and begins reading a verse.

### Go to bed [listened_to_father]
- @bed

### Talk to my younger sister Sophronia
- Sophronia says, "I think Father could preach at his own church if he wanted to. He knows more of the scriptures than anybody!"

### Talk to my younger brother Samuel
- Samuel says, "I get bored from scripture study sometimes, but I know it is important."

### Talk to my little brother William
- William says, "Hey Joseph, I've been practicing! I bet I could beat you at the stick game! Oh, yeah, I'm listening to father!"

### Talk to my little sister Katharine
- Katharine says, "Hey Joseph, will you tell me a story before bed? After we finish scripture study?"
- flag+ talked_to_katharine

### Play with my baby brother Don Carlos
- Don Carlos is only four years old. I rub his head, and he giggles and grabs my leg.

### Listen to Father
- Father says, "Here in the Book of James, we find a wonderful scripture: James chapter one verse five says: 'If any of you lack wisdom, let him ask of God.'" It is my testimony that our loving Father in Heaven always listens to our prayers.
- I notice that Alvin seems very deep in thought.
- flag+ listened_to_father

### Talk to Alvin [listened_to_father]
- I make my way to Alvin.
- "Oh Joseph," he says, "I wish all preachers would read that scripture in James. I wonder if any of them pray— I mean really, truly pray. I'm sure you know what I mean. I don't how they can say some of the things that they do."
- flag+ talked_to_alvin

## @room bed
Before I sleep, I pray beside my bed. I share my bed with Hyrum. I wish him goodnight. The bed is cool and comforting.

### Go to sleep
- @sabbath

### Visit Katharine [talked_to_katharine & !told_katharine_story]
- I quietly sneak out of my room. Katharine is right outside the door. "Oh Joseph, you remembered!" she says. "Just tell me a short one, okay?"
- @storytelling

## @room storytelling
- I think about the stories I know. Which one shall I tell Katharine?

### Tell her a story about Moses
- Once upon a time, there was an Egyptian king named Pharaoh. He was an evil man and ruled over the Israelite people in wickedness. But then God raised up a man named Moses. Moses was saved from a slaughter and God made him into a mighty prophet.
- Moses used the power that God gave him to afflict the Egyptians with plagues until they let his people go. But when the Israelites left, the Egyptians followed to take them back! Before long, they were stranded at the shore of the Red Sea.
- So Moses did something that no one had ever seen before: he smote the surface of the water with the rod that God gave him, and the sea parted, like a wall of water on either side! The Israelites passed through safely, but the water collapsed on the Egyptian army, destorying them. So Moses was able to save the people of Israel by the gift and power of God.
- Katharine says, "Thank you Joseph for that story. Do you think one day there will be a prophet like Moses again?"
- I think about that. "God could raise another one if he needed to. Now, off to bed Katharine. Goodnight."
- She says, "Goodnight, Joseph."
- flag+ told_katharine_story
- @bed

### Tell her a story about the Indians [talked_to_alvin]
- A story I have never heard before seems to rush into my head like a wind. "Once upon a time, there was a people banished to a faraway land. Their lives were like a bad dream— lonesome and solemn, wanderers⁠, cast out, born in tribulation, in a wilderness, and hated of their brethren. But they had a hope in their hearts that one day, the Son of God would come to visit them."
- Katharine's eyes seem to sparkle in the dark. "Did he visit them?"
- I pause, and a warm and mysterious feeling fills my bosom. "He did, Katharine. He appeared to them and showed them the wounds in his hands and feet. He brought the little children forth and prayed for them and blessed them. He brought the sick and afflicted and healed every one of them. He was so happy that they stayed righteous even though they had hard lives."
- Katharine looks down for a moment, deep thought weighing down her tender, seven-year-old brow. "Do you think that he will visit us one day?"
- I pause. "Jesus?"
- "Yes," says Katharine. "I hope that one day Jesus will visit us."
- I find myself hugging Katharine close. "Yes, He will visit us if we are righteous. Especially if we have hard lives. He will want to come to us and comfort us in our affliction. I know he will."
- "Now," I say, gently pushing Katharine away, "get to bed, Katharine. If we stay up any later we'll be weak and drowsy in the morning."
- She giggles gently. "Goodnight, Joseph."
- "Goodnight, Katharine."
- flag+ told_katharine_story
- flag+ told_indian_story
- @bed

### Tell her a story about a king and a queen
- I kneel down beside Katharine. "Okay, Katharine. Once upon a time there was a king and a queen. They ruled a vast kingdom that seemed to go on forever. But one day, the queen was riding in her carriage and noticed that some of the people in their kingdom were living difficult lives. They were cold and had little to eat."
- "What did the queen do?" said Katharine.
- "She decided to rebuild the entire kingdom. Working with the king, she drew out beautiful streets and buildings on a blueprint: a house of lodging for anyone without a home, a soup kitchen for anyone who was hungry, and warm fires on every street corner for anyone who was cold."
- "Did she build it?"
- "It took several years, but at last it was finally built. The people were happy again, and they loved God so much that they even wrote "Holiness to the Lord" on the bells of the horses!"
- Katharine giggled gently. "I want to live there!"
- I smiled. "I do too. But it is going to take a lot of work."
- "Do you think we can build it?"
- "Well, not by ourselves. But maybe one day we can get the people together and build a nice city. How does that sound?"
- "Sounds good!"
- I smile again at her cheerful face. "Now, let's get to sleep, okay?"
- "Okay. Goodnight, Joseph."
- "Goodnight, Katharine."
- flag+ told_katharine_story
- @bed

## @room sabbath
In the morning, we wake early and get ready for church. We put on our Sunday best. Half of my family goes to the Methodist church, and the other half doesn't identify with any church and goes where they feel like going. I'm in that second half.

### Go to the Methodist church with Mother
- @methodist

### Go to the Presbyterian church with Father
- @presbyterian

## @room methodist
Mother is happy that I am coming with her. We walk to one of the clearings where the Methodist itinerant preachers are wont to preach.

### Listen to the preacher
- The preacher says, "All of you, every one of you must take heed lest you be deceived! Only through the Holy Ghost can you know which way you must go. As James chapter one verse five says, "If any of you lack wisdom, let him ask of God." Which of you unaided can find wisdom? I say, no man!
- I am impressed at this preacher. And I am also stuck by that verse. I've heard it before.
- @ponder

## @room presbyterian
Father says, "I don't usually attend this church, but many of our neighbors go to one of the tall Presbyterian chapels down the road. Let's go with them this week."

### Listen to the preacher
- The preacher says, "Don't heed the authority of every preacher that comes your way. The house of God is a house of order indeed. Jesus says, "Take heed that no man deceive you, for many shall come in my name⁠, saying, I am Christ⁠; and shall deceive many...  And many false prophets shall rise, and shall deceive many!" You know there has recently begun to be much debate around religion in this area. To each of you, I plead: stay fast to what you know."
- I thought it odd that this preacher would question others' authority. After all, had he more authority than the rest of them?
- flag+ attended_presbyterian
- @ponder

## @room ponder
[!attended_presbyterian & talked_to_alvin]
I went home with mother deep in thought. What the preacher had said felt correct, but then I thought about what Alvin had said: "I wonder if any of them pray— I mean really, truly pray." Did the preacher really mean what he said, or was he merely reciting scripture? It it was impossible for a person young as I was, and so unacquainted with men and things, to come to any certain conclusion. I felt drawn to study and prayer.

[!attended_presbyterian]
I went home with mother deep in thought. In process of time my mind became somewhat partial to the Methodist sect, and I felt some desire to be united with them; but so great were the confusion and strife among the different denominations, that it was impossible for a person young as I was, and so unacquainted with men and things, to come to any certain conclusion who was right and who was wrong. I felt drawn to study and prayer.

[attended_presbyterian & talked_to_alvin]
I went home with father deep in thought. I thought about the things Alvin had said. How do preachers know what they know in the first place? It was impossible for a person young as I was, and so unacquainted with men and things, to come to any certain conclusion who was right and who was wrong. I felt drawn to study and prayer.

[attended_presbyterian]
I went home with father deep in thought. How was I to know which preachers were false and which were true? It was impossible for a person young as I was, and so unacquainted with men and things, to come to any certain conclusion. I felt drawn to study and prayer.

### Read from the Bible
- The verse in the Epistle of James, first chapter and fifth verse, keeps coming to mind. It reads: If any of you lack wisdom, let him ask of God, that giveth to all men liberally, and upbraideth not; and it shall be given him.
- Never did any passage of scripture come with more power to the heart of man than this did at this time to mine. It seemed to enter with great force into every feeling of my heart. I reflected on it again and again, knowing that if any person needed wisdom from God, I did; for how to act I did not know, and unless I could get more wisdom than I then had, I would never know; for the teachers of religion of the different sects understood the same passages of scripture so differently as to destroy all confidence in settling the question by an appeal to the Bible.
- @decision

### Pray
- As I pray, I feel the Holy Spirit. The verse from James keeps coming back to my mind.

## @room decision
I decided that I must either remain in darkness and confusion, or else I must do as James directs, that is, ask of God. I at length came to the determination to “ask of God,” concluding that if he gave wisdom to them that lacked wisdom, and would give liberally, and not upbraid, I might venture.

### Look outside [!decided_where_to_pray]
- The grove nearby my house is an excellent spot to pray. I decided that I would go there— tomorrow. It is getting late tonight. I had better get ready for bed.
- flag+ decided_where_to_pray

### Talk to Father
- I look at father and open my mouth, but no words come out. I have no idea what to tell him.

### Go to bed [decided_where_to_pray]
- @night2

## @room night2
I kneel beside my bed and pray earnestly, and it occurs to me for the first time that I have never prayed out loud before. I decided that tomorrow in the grove would be my first time. I was too shy to pray out loud where my brothers could hear me. At last we pull the covers up and sleep overtakes me.

### Wake up
- @wake_up2

## @room wake_up2
I wake up in the morning very early, before the rest of my family has really become active. It was a clear and beautiful spring day. It hadn't rained enough recently to form pools of mud. With any luck, no one in my family would even notice I had gone. 

### Leave for the grove [told_indian_story]
- @katharine_confrontation

### Leave for the grove [talked_to_alvin & !told_indian_story]
- @alvin_confrontation

### Leave for the grove [!talked_to_alvin & !told_indian_story]
- @grove

## @room katharine_confrontation
I head to the door, and I'm surprised to see Katharine there.

### Talk to Katharine
- "Hey Joseph," says Katharine.
- "Hey Katharine," I say. "What are you doing up?"
- "Well, I was thinking about the Indian story you told me last night. And then I dreamed about it. And I woke up early and felt like you were going to do something important today."
- I am not sure what to say to her. Either she has marvelous intuition, or God had given her some kind of hint.
- "I just want to say, good luck! I'll pray for you, okay?"
- "Thanks, Katharine," I say. "I really appreciate it. And I am doing something important. I'll tell you about it later, okay?"
- "Okay," says Katharine. "I'll leave you to it!"
- I smile at her and exit the house, closing the door behind me. It felt really good to have someone's support.
- @grove


## @room alvin_confrontation
I head to the door, and I'm surprised to see Alvin there.

### Talk to Alvin
- "Hey, Joseph," says Alvin. He shoots me a wry smile. He looks a little pale.
- "Are you okay, Alvin?"
- Alvin's smile flattens, and his eyes crinkle a bit. "I'm alright. I was praying really hard last night. I know that you know about my struggles with church and things like that. Well, I woke up early still thinking about it and still couldn't get back to sleep. So here I am."
- I wonder about Alvin's health. I wish there was some way I could help him.
- "Hey Joseph," Alvin says, "Where are you going?"
- I hesitate, feeling quite shy about my mission to go pray.
- "Well, whatever it is you are doing, do it well, eh?" Alvin smiles again. "I'll be alright. You go."
- I smile back, feeling a little worried for Alvin but grateful he was letting me go.
- @grove

## @room grove
[told_indian_story]
I walk down the path, entering into the woods. I know where I am going. The path stops abruptly, but I continue into the woods. A few weeks ago we went logging not too far from here. I remembered leaving an axe in a stump— I decided that would be my checkpoint. I half-wonder if the very forest itself knew what I was doing, after reflecting on Katharine's words, but the grove appeared to be totally indifferent. Nevertheless, the sunlight coming down through the leaves was nothing short of majestic.

[!told_indian_story]
I walk down the path, entering into the woods. I know where I am going. The path stops abruptly, but I continue into the woods. A few weeks ago we went logging not too far from here. I remembered leaving an axe in a stump— I decided that would be my checkpoint. The sunlight coming down through the leaves was nothing short of majestic.

### Kneel down
- I decide that now is as good of a time as any. I fall to my knees, and suddenly the weight of my question hits me. How could I wend my way through all this confusion? How can I really know that I am forgiven of my sins? I remember my determination to pray out loud. Now is the time.
- @temptation

### Check axe
- The axe looks like it is starting to get a bit rusty, unfortunately. I decide to leave it in the stump for now— I'll take it out when I'm done. Hopefully I won't forget it again.
- flag+ remember_axe

### Reflect on the desire of my heart [talked_to_alvin]
- I want to know which church to join. There are so many churches, and none of them seem to agree. Even my older brother Alvin seemed troubled by all the confusion. He always seemed invincible to me. If there is a chance I can find an answer through prayer, it is worth trying.

### Reflect on the desire of my heart [!talked_to_alvin]
- I want to know which church to join. There are so many churches, and none of them seem to agree. If there is a chance I can find an answer through prayer, it is worth trying.

## @room temptation
Information is what I most desire at this time, and with a fixed determination to obtain it, I call upon the Lord for the first time. But my attempt to pray was fruitless— my tongue seemed to be swollen in my mouth, so that I could not utter.

### Try to breathe
- @fear

### Try to speak
- @fear

## @room fear
I hear a noise behind me, like some person walking towards me. I try again to pray but I can't. The noise of walking seems to draw nearer. I spring up on my feet and look around, but I can't see any person or thing that could produce the noise of walking.

### Kneel down again
- @attack

## @room attack
As soon as I knelt again, I was seized upon by some power which entirely overcame me, and had such an astonishing influence over me as to bind my tongue so that I could not speak. Thick darkness gathered around me, and it seemed to me for a time as if I were doomed to sudden destruction.

### Cry out to God
- Finally, words come out. It came out almost as a scream: "Oh Lord, my God!"
- Suddenly, as soon as I was attacked, I was delivered from the enemy which had had me bound. I see a bright light— a glorious light— a pillar of light that is brighter than the sun at noon-day.
- It seems to fall upon me and rest on my head— and then completely envelop me. It seems like the whole forest is on fire, but I don't feel alarmed. I feel love and peace in my heart. Above me I see countless angels singing and raising their voices to God.
- @vision

## @room vision
The heavens seem to open, and the light somehow grows brighter. Out of the light, I see two people whose brightness and glory defy all description. I want to weep and praise them like all the other angels, but I can't do anything but be still in this magnificent light. The one on the right speaks to me: "Joseph, this is my Beloved Son. Hear Him!"

### Hear Him
- "Joseph, my son, thy sins are forgiven thee. Go thy way, walk in my statutes, and keep my commandments. Behold, I am the Lord of glory. I was crucified for the world, that all those who believe on my name may have eternal life."
- My heart is filled with unspeakable joy. I know, now and forever, that this is truly the Christ. I managed to exercise courage and finally ask my question: "Lord, which church shall I join?"
- "Behold, the world lieth in sin at this time, and none doeth good, no, not one. They have turned aside from the gospel and keep not my commandments. They draw near to me with their lips while their hearts are far from me. And mine anger is kindling against the inhabitants of the earth, to visit them according to their ungodliness and to bring to pass that which hath been spoken by the mouth of the prophets and apostles."
- "Therefore, thou must go not after them. But in mine own due time, thou shalt receive a fulness of my gospel. Behold and lo, I come quickly, as it is written of me, in the cloud, clothed in the glory of my Father."
- @ground

## @room ground
He tells me many things concerning the future of the kingdom— I could never write it all. The vision finally closes and I find myself lying on my back, looking up into heaven. When the light had departed, I had no strength. But I recover soon enough and decide to head home.

### Head home
- @go_home

### Grab axe [remember_axe & !have_axe]
- I had nearly forgotten the thing. It doesn't seem anywhere near as important as it did before, but I decided to take it anyway.
- flag+ have_axe

## @room go_home
[talked_to_alvin | told_indian_story]
As I am walking home, I see mother at the gates. "Oh Joseph!" she exclaims, "Are you alright? You look so pale!"

[!talked_to_alvin & !told_indian_story]
As I am walking home, I see the house, as quiet as ever. The trees are still magnificent and indifferent. I am still marveling at the majesty of the vision.

### Speak with mother [talked_to_alvin | told_indian_story]
- "Let us get you to the fireplace. Here, come inside," she says. I oblige.
- As I am leaning against the fireplace, she asks, "Joseph, what happened? What is the matter with you?"
- I reply, “Never mind, all is well—I am well enough off. I have learned for myself that Presbyterianism is not true.”
- end

### Head inside [!talked_to_alvin & !told_indian_story]
- I head inside and lean against the fireplace. Mother is in the room, and she looks over at me. "Joseph, are you all right? What is the matter?"
- I reply, “Never mind, all is well—I am well enough off. I have learned for myself that Presbyterianism is not true.”
- end