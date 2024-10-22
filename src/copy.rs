/*!
 * Author: Colton Philips
 *
 * Description:
 *
 * &Str storage of copy text within the game allowing for ease of edit and access.
 * Prefer absense of "magic unnamed variables" in logical code
 *
 * Usage:
 *
 * Notes:
 *
 */
pub const SKIPPABLE_INTRO: &str = "
    Tiny Creatures Support Group! 
               by Colton Phillips
    =============================
 
    You find yourself at a support group
    for SMALL CREATURES with 20 bucks! 
 
 It's your job to greet each member, who
 may be a small animal, sentient creature,
 or some mythic or pop culture character.

 BUT... You forgot to bring any nametags!
 And you can't remember everybody's name!
";
pub const RULESET: &str = "
 You must try to remember the name of each creature(s):
 
 (e.g. 'PUPPIES' 
 has 3 'P's, 1 'U', and 1 'Y')
 (e.g. 'TWEETY'
 has 2 'T's, 2 'E's, 1 'W', and 1 'Y')

 Each creature will give you errant thoughts
 when looking on them: (e.g. \" This guy is cool! \" )

 Don't make too many mistakes or people
 will think that you're a bit of a narcissist.
 Guess the creature's name ONE letter at a time.
 Type 'BUY' to BUY-sect the unused guesses. Costs {bisect_cost}
 Type 'QUIT' to leave at any time.
 
 Press Enter to greet the first member
 of the Tiny Creature Support Group (I hope they brought snacks!)
 ";
