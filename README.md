# Gold Rush
## Description
My solution for Gold Rush competition https://cups.mail.ru/en/tasks/1057

## Conditions
For a long time we have not heard any adventure stories about treasure hunters ... And all because the time of unexpected adventures, unfortunately, is over!

Everything is now under account, banks control all excavations, order and equality of opportunity reign in the world.

But even here there is room for risk and opportunity to get rich quickly! The key is to choose the right strategy and be ready to adapt to change.

Imagine you are traveling the world with other treasure hunters. Coming to the excavation every time you have one goal - to collect as much gold as possible in the allotted time and not give competitors a chance to win. At the same time, over the entire time of travel, your equipment has become very worn out and can no longer work as fast as before, it can glitch and freeze in operation.

## Description of the task
You are given a three-dimensional playing field measuring 3500x3500 cells, in which treasures are buried at a depth of 1 to 10 floors.

The main task of — is to get as many coins as possible in the allotted time (see the restrictions on the solution).

You need to write an application packaged in a Docker-type container that meets the requirements of the task and is able to make requests to the API of our server - the game world.

## Generating the world
Technically, the organizers of the competition have developed a server application that works according to a specific business logic.

When generating the world, the parameter is used SEED. It determines how many treasures and on which floor will be generated, what will be the value of each treasure and which floor will be optimal in terms of the ratio of the value of the treasures to the cost of their extraction.

In addition, the game world mimics the environment. You can come across different errors that depend on the time (CPU_TIME) and RPS of the game world, and specifically CPU_TIME and RPS for each ENDPOINT of the game world. The probabilities of errors depend on this, and you need to investigate them.

Please note that the server source code and the parameter SEED during the competition will not be published.

This round is guaranteed to be:

490,000 treasures;
23,030,000 coins are in the hoards.

## The essence of the game world
Treasure (what you need to find and gives coins);
Licenses (what allows us to dig), which are divided into two categories - free and paid;
Coins (an array of coins that allows us to buy licenses).

## Game process
You need to find treasures in the playing field, which is essentially a three-dimensional space. To find the treasure, you need to dig, and you can only dig in the depth. A prerequisite for the operation "dig" is the presence of an active license, which are free or paid.

An active license must have a positive value for the number of digs, so they can be used several times for digging, while the number of digs for free licenses is 3, and for paid licenses it is always different and depends on the number of coins spent on purchasing a license. The license is deactivated after the entire number of copies has been exhausted.

You can buy licenses for coins that you receive after cashing out the treasure.

## Game world API
For searching you are given an API with POST request /explore, allowing to explore the field in X, Y coordinates. According to these coordinates, on a certain floor, treasures can be buried that contain a certain amount of coins. The larger the search area, the longer it will take to explore it and the higher the probability of error.

API request POST /dig, which takes the coordinates and ID of the license, allows you to dig depth 1 floor down. If there is a treasure at the given coordinates and at the current depth, then you will dig it, reduce the number of digging for the active license and dig down 1 floor. If there was no treasure, then you will receive a 404 error, reduce the number of digging for the active license and also dig down 1 floor. To dig up the treasure on the 10th floor, you need to sequentially dig the remaining 9 floors. You cannot dig to the sides. Dug cells are always dug. The deeper you dig, the longer it takes for one API call. As you move deeper, the number of coins in each hoard increases. On the 10th floor, it reaches its maximum value. But, it should be borne in mind that with different parameters of the game world SEED the optimal floor in terms of the ratio of the value of the treasure to the cost of its extraction will change.

You can get a license via API by request POST /licenses. To get a free license, you need to transfer to the API by request POST /licenses an empty request, and for a paid license, you need to transfer more IDs of the coins that you want to spend on its purchase. One coin cannot be spent twice. The free license always allows you to dig deep 3 times and has an increased chance of getting 5хх errors when requested. A paid license provides an opportunity, according to a certain pattern, to get N-digging for X the number of coins that you spend on its purchase. The price list of the coins is unknown and you'd better determine it.

Balance is the length of the coin array. To clarify the balance, there is an API with a GET / balance request, but it is not necessary to use it. Balance is a metric for the effectiveness of your solution. The balance is equal to the number of points earned (SCORE).

ОYou can exchange found treasures using the POST / cash API. As a result of the exchange of treasures, you get coins, which are reflected in your balance according to the results of your solution. Please note: if you spend earned coins to buy paid licenses, then your total SCORE decreases accordingly.

We draw your attention once again to the fact that in the game world a certain probability of receiving errors for any request is deliberately laid. In some situations, it will be useful for you not to wait too long for an answer, but to try to repeat the request again.

Your solution will be most successful if you can do several different things at the same time. But it should be borne in mind that sometimes very frequent requests have a negative effect.

Full API description is available in the file swagger.yaml in the championship repository.

## Game result
The result of the game is the balance at the end of the game session, it is also SCORE.

## Technical information
To start, the service must be wrapped in a Docker container.

Transmitted environments:

ADDRESS: default
Port: 8000;
Schema: http

## Limitations on solutions
In this round:

— your decision is given 10 minutes to complete. Manage the allocated time yourself.

— we are not asking you to build a solution in a container, you can build it locally and send it in a container already as a compiled file, but please note that in the final round, you will need to build a solution in a container.

— we have not set a limit on the download of solutions per day. Depending on the load on the calculation system, we can limit the download of solutions to 50 per day per user.

Each solution is guaranteed resources:

Cores: 4
RAM: 2048 MB
Swap: 2048 MB
Timeout: 900 Sec
The server you are making requests to has technical characteristics:

Language: golang
Cores: 2
RAM: 2048 MB
SWAP: 2048 MB
Timeout: 900 Sec
You must check the server yourself whether it is ready or not. If the server does not receive requests for key endpoints within 900 seconds, the game session ends.

Good luck!
