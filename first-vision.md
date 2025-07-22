# The First Vision: The Text Adventure

@start starting_room

## @room starting_room
My name is Joseph Smith. I was born in the year of our Lord one thousand eight hundred and five, on the twenty-third day of December, in the town of Sharon, Windsor county, State of Vermont. My father, Joseph Smith, Sen., left the State of Vermont, and moved to Palmyra. We were, by necessity, required to obtain a scanty maintenance through our daily labor. I am at the farm with my brothers, working to gather wheat into large bales.

### Bale some hay
- "After considerable effort, I have made another bale of hay."
- count+ hay_bales

### Talk to Alvin
- "Alvin says, "Hey, Joseph! Looking strong today! Think you can bale some hay?""

### Talk to Hyrum
- "Hyrum says, "Don't worry, Joseph, I know you still struggle with your limp sometimes. I can bale hay for us if you are feeling tired.""

### Go home [hay_bales > 2]
- @home

## @room home
After a hard day of work, it is nice to be inside. The daylight grows dim, and we light up our lanterns. Father pulls out his reading glasses and begins reading a verse.

### Finish
- end

### Listen to Father
- "Father says, "Here in the Book of James, we find a wonderful scripture: James chapter one verse five says: 'If any of you lack wisdom, let him ask of God.'""
