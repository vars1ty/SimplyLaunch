# SimplyLaunch
An alternate, simple horse game launcher, because over-engineering one using IPC and Electron bloat is all they can do themselves.

## Why?
Because I don't want to sit and wait for their horrible pasta-coded launcher takes its sweet time to load useless stuff, like:
- Star Coins
- Stable Care
- Offers
    - No joke, they got a whole damn sub-section in their API just for this
- A whole-ass video
- Useless CSS which, by the way, **you'll only look at for 5 seconds until the game launches, nobody sits and stares at your Windows 8-like clustered launcher**
- A billion different libraries, many of which they could of easily have re-created themselves.
    - Like grabbing your Machine ID, how can a million-dollar company fail to implement something so basic, on a launcher that's only exported for **2** platforms?
    - You only have to find a way of grabbing the ID on 2 separate platforms, and don't worry about Linux since WINE/Proton will to 99% work just fine with fetching it too.

In conclusion; their launcher is a bloated, slow and utter mess. Just extract the asar and check for yourself, I'd be surprised if you came back with any sanity left.

Oh and Linux. Just use the normal launcher to download the game/update it and then this. Less headaches from dealing with the incompetent company's own stuff.

## Does it implement everything the actual launcher has?
No, not even close to it.

It's meant as an replacement for their own launcher when you have already downloaded the game files/updates and so on, nothing more.

It also doesn't implement the Login Queue (as if you'd ever get in that, the game's dead) and only fetches it to get the mandatory "Queue Token".

Whether or not I'll implement support for the queue depends, but I have never gotten it more than once, and that was when it first released.

Regarding downloading game files/updates, highly unlikely for several reasons:
1. Searching through their awful code to reverse-engineer how they download files and then debug it all will leave me crippled, although downloading game files would probably be the easiest step out of these 2 points
2. I have no idea how they patch their game files, whether it's proper patching or just re-downloading the .CSA/.CSAHEADER files.

## Where's the UI?
There's none, it's a terminal application. I'm not implementing a whole UI for something this simple.
