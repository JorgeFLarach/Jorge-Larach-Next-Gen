use std::io;

#[derive(Debug)]    
enum QuestionType {
    MultipleChoice(Vec<String>, usize), 
    TrueFalse(bool),                    
}

#[derive(Debug)]    
struct Question {
    question: String,
    question_type: QuestionType,
}

impl Question {
    fn new_question(question: &str, question_type:QuestionType) -> Question { // Associated Function
        Question {
            question: question.to_string(),
            question_type,
        }
    }

    fn ask(&self) { // Method
        println!("{}", &self.question);
        match &self.question_type{
            QuestionType::MultipleChoice(options, _) => {
                for(i, option) in options.iter().enumerate(){
                    println!("{}. {}", i+1, option);
                }
            }

            QuestionType::TrueFalse(_) => {
                println!("1. True");
                println!("2. False");
            }

        }
    }

    fn check_answer(&self, guess:usize) -> bool { // Method
        match &self.question_type {
            QuestionType::MultipleChoice(_, ans) => guess == ans+1,
            QuestionType::TrueFalse(ans) => match guess {
                1 => *ans,
                2 => !*ans, // if solution is true, selecting false inverts the solution (returns false, wrong answer)
                _ => false
            }
        }
    }
}

fn main(){
    let quiz = vec![
        Question::new_question("What is the best Mexican place in San Antonio?", 
            QuestionType::MultipleChoice(vec!["El Milagrito".to_string(), "Chelas".to_string(), "El Regio".to_string()], 0)),

        Question::new_question("I understand ownership", QuestionType::TrueFalse(false))
    ];

    let mut score = 0;

    // Quiz loop
    for question in quiz.iter() {
        question.ask();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess = guess.trim().parse::<usize>();
        
        match guess {
            Ok(guess) => {
                if question.check_answer(guess) {
                    println!("correct!");
                    score += 1;
                } else { println!("*buzzer sound*"); }
            },
            Err(_) => println!("not a number")
        }
        println!("\n");
    }
    println!("Quiz completed. Your score: {}/{}", score, quiz.len());
}