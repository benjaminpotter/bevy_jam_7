# Bevy Jam #7

A turn-based management deckbuilder where you play a maniac doctor deliberately inducing fever delirium in patients with minor injuries.
Patients arrive with mundane ailments, but your true goal is to maximize their Delirium score before they leave—without killing them or attracting too much suspicion. 
You’re not curing illness; you’re cultivating fever dreams.

Play cards to manipulate:
- Delirium (your score)
- Health (hard fail if it hits zero)
- Stability (resistance to delirium)
- Suspicion (clinic-wide pressure)

- Waiting room shows upcoming patients
- Each turn player sees the next patient
- They draw from their deck to fill their hand
- Player keeps remaining cards from last turns
- They have a certain number of action points per turn
- Each card costs action points
- Player needs to cause every patient to have a fever dream
- Suspicion meter fills the player loses
   - Killing patients raises suspicion meter


## TODO

- [ ] Configure itch.io releases in CI
   - Need an itch.io project target
   - See CI.md
- [ ] Patients
- [ ] Cards
