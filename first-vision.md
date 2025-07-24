# The First Vision: The Text Adventure

@start starting_room

## @room starting_room
My name is Joseph Smith. I was born in the year of our Lord one thousand eight hundred and five, on the twenty-third day of December, in the town of Sharon, Windsor county, State of Vermont. My father, Joseph Smith, Sen., left the State of Vermont, and moved to Palmyra. We were, by necessity, required to obtain a scanty maintenance through our daily labor. I am at the farm with my brothers, working to gather wheat into large bales.

### Bale some hay
- After considerable effort, I have made another bale of hay.
- count+ hay_bales

### Talk to Alvin
- "Alvin says, "Hey, Joseph! Looking strong today! Do you think you could do at least three bales of hay?"

### Talk to Hyrum
- "Hyrum says, "Don't worry, Joseph, I know you still struggle with your limp sometimes. I can bale hay for us if you are feeling tired.""

### Go home [hay_bales > 2]
- @home

## @room home
After a hard day of work, it is nice to be inside. The daylight grows dim, and we light up our lanterns. Father pulls out his reading glasses and begins reading a verse.

### Go to bed
- @bed

### Listen to Father
- Father says, "Here in the Book of James, we find a wonderful scripture: James chapter one verse five says: 'If any of you lack wisdom, let him ask of God.'"

## @room bed
Before I sleep, I pray beside my bed. I share my bed with Hyrum. I wish him goodnight. The bed is cool and comforting, and I have a deep, dreamless sleep.

### Wake up
- @sabbath

## @room sabbath
In the morning, we wake early and get ready for church. We put on our Sunday best. Half of my family goes to the Methodist church, and the other half doesn't identify with any church and goes where they feel like going. I'm in that second half.

### Go to the Methodist church with Mother
- @methodist

### Go to the Baptist church with Sophronia
- @baptist

## @room methodist
Mother is happy that I am coming with her. We walk to the Methodist church just down the road.

### Listen to the preacher
- The preacher says, "All of you, every one of you must take heed lest you be deceived! Only through the Holy Ghost can you know which way you must go. As James chapter one verse five says, "If any of you lack wisdom, let him ask of God." Which of you unaided can find wisdom? I say, no man!
- I am impressed at this preacher. And I am also stuck by that verse. I've heard it before.
- @ponder

## @room baptist
Sophronia leads me a long way to the Baptist church.

### Listen to the preacher
- The preacher says, "The Bible is clear on this matter— each and every one of you must be baptized in the name of Jesus Christ. Countless millions of the heathen burn in hell because they never were baptized. How fortunate are we to have the opportunity for baptism And how cursed are we if we reject this opportunity!
- I wonder where the preacher received authority to baptize. Though it is true that the Bible makes frequent mention of baptism.
- @ponder

## @room ponder
In process of time my mind became somewhat partial to the Methodist sect, and I felt some desire to be united with them; but so great were the confusion and strife among the different denominations, that it was impossible for a person young as I was, and so unacquainted with men and things, to come to any certain conclusion who was right and who was wrong.

### Read from the Bible
- The verse in the Epistle of James, first chapter and fifth verse, keeps coming to mind. It reads: If any of you lack wisdom, let him ask of God, that giveth to all men liberally, and upbraideth not; and it shall be given him.
- Never did any passage of scripture come with more power to the heart of man than this did at this time to mine. It seemed to enter with great force into every feeling of my heart. I reflected on it again and again, knowing that if any person needed wisdom from God, I did; for how to act I did not know, and unless I could get more wisdom than I then had, I would never know; for the teachers of religion of the different sects understood the same passages of scripture so differently as to destroy all confidence in settling the question by an appeal to the Bible.
- end

### End
- end