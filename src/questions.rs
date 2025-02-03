/*

*/

#[allow(dead_code)]
pub struct Question {
    pub prompt: String,
    pub good_option: String,
    pub bad_option: String,
    effect: f64,
    chosen: Option<bool>,   // True for option 1, False for option 2
    pub used: bool
}

impl Question {

    fn new (
        prompt: &str, 
        good_option: &str, 
        bad_option: &str, 
        effect: f64,
    ) -> Self {
        Question {
            prompt: prompt.to_string(),
            good_option: good_option.to_string(),
            bad_option:  bad_option.to_string(),
            effect,
            chosen: None,
            used: false
        }
    }

    fn choose (&mut self, choice: bool) {
        self.chosen = Some(choice);
        self.used = true;
    }

    pub fn choose_option (lit_percentage: f64, question: &mut Question, chose_good: bool) -> f64 {
        match chose_good {

            true  => { // chose the moral option, add effect
                question.choose(true);
                lit_percentage + question.effect
            },

            false  => { // chose the immoral option, subtract effect
                question.choose(false);
                lit_percentage - question.effect
            }

        }
    }
}

pub fn give_questions (num_questions: usize) -> Vec<Question> {
    let mut questions: Vec<Question> = vec![
        /* //! FORMAT FOR THE THING:
        Question::new(
            prompt,         //? where the scenario is set
            good_option,    //? Text for the good option
            bad_option,     //? text for the bad option
            effect,         //? the moraility score effect on the player (between 0 and 1)
        )
        */

        Question::new(
            "The passage to your left leads more quickly to your goal. Turning right will reunite a missing person with their family.",
            "Reunite the missing person with their family",
            "Save yourself some time, go the fast way",
            0.1
        ),
        
        Question::new(
        "Your grandmother just found out she has breast cancer and it is at an early enough stage to stop it, but barely. However, she does not have enough money to pay for the procedure, nor does anyone else in your family. You are walking down the street and see a tech CEO walking out of the bank, having just made a withdrawal. You can either mug the CEO, leaving them with psychological trauma, and use the money to get your grandma treated, or you can start a GoFundMe, but you might take too long to raise sufficient funds.",
        "Start a GoFundMe, and hope you get the money in time",
        "Go mug a CEO, lend some of your life for the rest of hers",
        0.5
        ),
        
        Question::new(
            "You are going grocery shopping. Your local small-town grocery store gets its food shipped in from halfway around the world from sources known to use slave labour. There is a bigger, more successful supermarket that only sells ethically sourced goods. You must choose whether to support a local business, but also by extension slavery, or to buy from a big company that gets better foods.",
            "Get the ethically sourced food",
            "Support the local business, but also slavery",
            0.5
        ),
        

    ];
    
    let num_questions_but_real: usize = if num_questions >= crate::MAX_QUESTIONS {crate::MAX_QUESTIONS} else {num_questions};

    if num_questions_but_real < questions.len() {
        // remove however many elements are excess
        for _ in 0..(questions.len() - num_questions_but_real) {

            //? Don't ask I stole it from StackOverFlow, it just removes a random element.
            let index = (rand::random::<f32>() * questions.len() as f32).floor() as usize;
            let _ = questions.remove( index );
        }

        questions
    }
    else {
        questions
    }
}