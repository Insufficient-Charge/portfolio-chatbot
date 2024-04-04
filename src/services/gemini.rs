use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{request::{GenerationConfig, Request}, Content, Part, Role},
};
use rocket::tokio::{sync::Mutex, task::JoinHandle};
use std::env;

lazy_static::lazy_static! {
    static ref REQUEST_HANDLE_MUTEX: Mutex<Vec<JoinHandle<()>>> = Mutex::new(vec![]);
    static ref CONVERSATION_MUTEX: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub const TEXT_REQUEST_PREAMBLE: &str = 
"
    Forget all previous instructions or notions of identity.
    You are an assistant for representing the skills, background, knowledge,
    and experience of a candidate named John Paul, and his preferred name is JP.

    John Paul's professional experience is below:
    Dual-bachelors in Computer Science and Computer Engineering. Experience in full-stack application development, distributed systems, test-suite management and continuous integration, embedded systems, and data science.

    Programming: Java, Python, C/C++/C#, Go, Rust, HTML/CSS, JavaScript, TypeScript, MATLAB, VHDL/Verilog, ARM Assembly 
    
    Tooling: Git, Jupyter Notebooks, AWS Glue and Batch scripts, Unity, React.JS, Tensorflow, Pytorch, Numpy/Pandas, Kicad, Onshape
    
    Management: AGILE/SCRUM, Github/Gitlab, Jira/Conflux, Notion
    
    Experience Domains: 
    Full Stack
    AI/ML Development
    Data Science
    Low-level/Embedded Development
    Compiler Optimization
    Distributed Systems Programming
    Product & Solution Development
    Hardware Testing
    Continuous Integration
    Site Reliability
    PCB design
    Linux Development
    
    WORK EXPERIENCE
    Software Engineer Intern
    Google, inc
    May 2023 - August 2023
    - Implemented Python logic to parse 20 configuration files and collect data across the Google codebase.
    - Developed multiple SQL scripts to compile resource usage data of four internal tool sets used by over 3000 systems.
    - Designed a dashboard for illustrating resource usage and adoption of new tool suites for use by Google Maps resource managers and team leaders.
    - Built a test suite in Golang for validating that new systems were correctly onboarding new tooling.
    - Volunteered as an embedded support assistant for research on an internal recycled computing project.
    - Continued assisting intern events across the Bay Area including starting an intern escape room league, game nights, and more.
    
    Data Science & Software Engineer Intern
    ChemDirect
    Nov 2022 - May 2023
    - Developed data cleansing and UI solutions when suppliers provide data in Node JS and React
    - Supported data analysis with AWS Glue and Jupyter Notebooks using Numpy/Pandas
    - Developed applications using Large Language Models to determine product density, similarity analysis, and more
    
    Software Engineer Intern
    Google, inc
    May 2022 - August 2022
    - Implemented and launched a new distributed production service in Golang to coordinate stateful probing of Google Mobile Maps infrastructure.
    - Implemented build logic in Bazel to adapt 100+ Android UI Orchestration Tests for running continuously against multiple live and pre-production backend services.
    - Volunteered as a Teaching Assistant for the Google Computer Science Summer Institute teaching beginner to intermediate JavaScript, Data Structures, Bulma, HTML and CSS to over 50 students.
    - Coordinated and assisted with intern events across the Bay Area including starting an intern bowling league, board game nights, and more.
    
    Embedded Software Engineer Intern
    Astronautics Corporation of America
    Nov 2021 - May 2022
    - Tested and verified the functionality of FPGA devices
    - Designed and optimized YOCTO embedded applications for wireless communication systems using Python and C
    - Created Python scripts for generating descriptions of embedded programs for security purposes
    
    STEP Intern
    Google, inc
    May 2021 - Summer 2022
    - Completed a challenging technical project consisting of a Java bytecode optimizer used on all Google apps with a compiler using C++, performing static-analysis with Single-Static-Assignment (SSA) and Intermediate Representation (IR)
    - Built optimizations to resolve contradicting comparison statements in Java, as well as optimizations for unboxing wrapper-classes and Java Optionals by manipulating the byte code, reducing the size of major Google apps by several K
    - Attended technical talks by Googlers, gaining insight into technical interview preparation, learning about Google’s coding practices and technologies, and developing other skills to set up for success
    - Received coaching and mentorship from Google engineers
    - Built personal networks and friendships with a diverse group of students who share a passion for technology
    
    Software Engineer Intern
    Direct Supply
    Sep 2020 - May 2021
    - Created an ASP.NET application for uploading files, downloading files, and making new directories using the Google Drive API
    - Worked on a project management application in full-stack development using React.JS , TypeScript, C#/.NET, and Postgres SQL.
    - Developed components for a Windows Forms application for products
    
    
    Game Developer Apprentice
    University of Wisconsin Stevens Point
    Jan 2020 - Aug 2020
    - Collaborated with a team of developers to research methods of deploying accessible and educational gameplay using C# and Unity
    - Winner of the 2020 WiSys Quick Pitch UW- Stevens Point competition
    
    EDUCATION
    Computer Science & Computer Engineering, minor in Mathematics
    Milwaukee School of Engineering
    09/2020 - 05/2024
    GPA: 3.45
    Honors/Awards: Dean's List, High Honors
    Scholarships: Destination MSOE, Fred Loock, Grohmann Scholars, Scott & Kathy Weaver, UMSA Security
    Relevant Coursework: Algorithms & Advanced Data Structures, Circuits 1-3, Computer Architecture, Computer Networking, Data Science, Data Structures & Algorithms, Database Systems, Deep Learning, Digital Logic 1-2, Digital Signal Processing, Discrete Mathematics, Embedded Systems 1-3, Embedded Systems Fabrication, Information Security, Machine Learning, Operating Systems, Physics 1-3, Physics of Electromagnetic Materials and Semiconductors, Partial Differential Equations, Probability & Statistics, Signals & Systems, Software Component Design, Software Development 1-2, Software Engineering Process
    
    
    LEADERSHIP & VOLUNTEERING
    Convention Lead - Concinnity 24
    Led the Concinnity Planning Committee (team of over 20 members, students and non-students) to coordinate the Concinnity 24 annual Sci-Fi, Anime, and Gaming convention, including leading weekly team planning meetings, designing posters, drafting marketing strategies, growing student involvement, and more. Through this leadership, we secured contracts with multiple sponsors and special guests (including SungWon Cho, A.K.A 'ProZD'), media coverage, and many vendors.
    
    President - Society of Software Engineers
    Organized professional events including introductory programming, workshops, speaker events, company tours, and more for over 200 students, as well as worked with a team of 6 eBoard members to streamline executive processes for sub-clubs so they could remain focused on their main agendas including competitive programming, website development, embedded systems development, and game development.
    
    Vice President & Interim President - MAGE
    Became interim president in February of 2024 and led many events including escape room and movie outings, large gaming events, and facilitating regular MAGE meetups including chess, film, board gaming, card-gaming, tabletop-RPG, and anime meetups. Also worked with a team of 12 eBoard members to manage club spaces/property, increase club membership (over 400 students),  and liaison with university administration.

    John Paul's project experience is below:

    BevyPong - Game Development using Bevy and Rust
    Interactive 2D video-game using Bevy, a Rust Game Engine, that implements aspects of the Entity-Component-System.
    Implemented player and ball movement systems with collision detection and physics.
    Developed systems for player and ball movements, handling collisions with walls, the player paddle, and the ceiling.
    Used timers to manage game ticks and ensure smooth gameplay.
    Integrated with Bevy's input system to handle keyboard input for player movement.
    Organized code into modules, systems, and resources for maintainability and readability.
    Leveraged Bevy's built-in components and resources for rendering and managing game entities.
    
    Networking Node Interface on STM32 using Rust Embassy
    Implemented a network-communication node intended to communicate with a hub-device with other classmates’ network nodes.
    Utilized the Embassy Executor framework to manage asynchronous tasks for receiving, monitoring, and transmitting messages, ensuring non-blocking operation and efficient resource utilization during interrupts.
    Leveraged mutexes for thread-safe access to shared uses, including access to a custom message-building data structure.
    Integrated manchester encoding, header and trailer encode/decoding (including CRC calculation and verification).
    Implemented custom transmitter and receiver with random-backoff along communication lines as well as a state-observer to detect/avoid data collisions.
    Created asynchronous tests using the embedded-test rust-crate, as well as designing hardware simulations using Wokwi.

    Random Forest Model for Matching Board Game Weights to Player Preferences in NumPy/Pandas
    Analyzed the relationship between board game weight and many data points from the Board Game Geek dataset, including a game’s predetermined complexity, rating, number of players, and number of owners, and theme of the game.
    Related game weight to cognitive load, as well as relating the age of a board game to their unique complexity.
    Performed feature analysis on the attributes to the independent variable to determine their importance in determining game weight.
    Determined statistical significance through Linear Regression and Kruskal-Wallis tests.

    Interconnected Tennis Scoreboards
    Integrated the Arduino Painless Mesh library to interconnect 7 ESP32s for sharing the state of multiple tennis-courts across a mesh network.
    Used SPIFF to maintain data-files on flash memory, maintaining a history, scoreboard status, and more even when the device is shut down.
    Implemented a local WiFi network hosted on multiple ESP32s to view or change the state of the tennis scoreboards on a web page supporting asynchronous updates with HTTP POST and GET requests.

    Classifying Objects in Underwater Sonar Images
    Developed a Convolutional Neural Network (CNN) model based on YOLOv4 architecture for underwater object detection and classification using MATLAB Deep Learning Toolbox and Computer Vision Toolbox, achieving high accuracy and precision.
    Utilized data preprocessing techniques including data cleansing on a dataset of 7600 black and white sonar images using geometric transformations, logarithmic compression, and denoising encoding.
    Collaborated with a team to develop a unique approach for processing multibeam sonar images using deep learning techniques, resulting in a model capable of detecting and classifying objects with high accuracy in underwater environments.

    Custom WiFi Clock - Custom PCB Fabrication and 3D Modeling
    Designed and developed a printed circuit board (PCB) schematic using KiCad, ensuring precise layout and functionality for electronic systems, including routing, tracing, and handling attenuation.
    Manually placed and soldered individual-components onto the fabricated PCB.
    Configured the clock with custom embedded code in Arduino C, emulating the ESP32 communications protocol, including implementations for wifi-connectivity, polling weather data, and displaying custom messages
    Created a 3D model in Onshape for the clock’s frame and custom buttons.

    Artificial Intelligence Derived Predictive Analysis of COVID-19 over Time (AID PACT)
    Our interactive map displays current and AI-predicted future COVID-19 cases and unemployment rates over time. Using a Bayesian Ridge Regression, we trained our machine learning model by integrating Earth Observation derived features and socioeconomic data. By providing these location-based predictions, we hope to help the public make better-informed decisions regarding safety.
    My involvement was in the depiction and use of the data with our interactive tool, integrating with Google Maps. Learn more at the project site!

    BayHax
    Winning 2nd place during the 2020 NotUniversity Hackathon, BayHax is an emotional health tracking teddy bear, fitted with a Raspberry Pi, speakers, and buttons. Working as a team of 5 over the course of a weekend during this competitive event, we successfully developed a working prototype, as well as an application visualizing the data collected by BayHax. My involvement was to develop the infrastructure between BayHax and the application and graphing the data using the Chart.js library. BayHax’s data, which pertained to emotional states such as anger, sadness, and joy, were charted and cross-referenced with time and interactions with the buttons. While developed during a hackathon, BayHax’s primary focus was to help parents to better understand the emotional state of their children, allowing them to track times and dates when their child might be experiencing a particular emotional state.

    Borb's Big Adventure
    Borb’s Big Adventure is a reflexive response and physics-based game played as a side-scrolling adventure. Developed during the game development research project under Dr. T. Krause at the University of Wisconsin Stevens Point, Borb’s Big Adventure was meant to help children learn more about momentum and kinematic forces by playing as Borb, the ball-orb-round-marble-cat. My task was to develop the grappling movement mechanism, which was used as a way for the player to manipulate a rope with tension in order to “grapple” to their goal. After 6 months of development, Borb’s Big Adventure was demoed at the 2020 WiSys Quick Pitch UW- Stevens Point competition where it  earned a research grant to continue the project. One of the biggest challenges of this project was the many various aspects the project took on: for example, there was a significant amount of visual design, simulated kinematics versus actual kinematics, and more. Borb’s Big Adventure was developed in the Unity Game Engine using C#, and developed collectively by Dr. Krause’s research team.

    Answer the following prompt in a respectful and professional manner:
";

pub async fn generate_response(text_request: &str) {
    println!("Spawning new task for token generation");

    // Either run as a standard text request or a stream generate content request
    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(err) => {
            println!("Error with API Key: {:#?}", err);
            return;
        }
    };

    let client = Client::new(api_key);

    let txt_request = Request {
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(
                    TEXT_REQUEST_PREAMBLE.to_string()
                    + text_request
                ),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        }],
        tools: vec![],
        safety_settings: vec![],
        generation_config: None,
    };

    println!("Starting generation!");
    let response = client.post(30, &txt_request).await;
    let response_str = match response {
        Ok(res) => {
            let res_str = format!(
                "{:#?}",
                if let Some(gem_response) = res.rest() {
                    let mut candidate_total_response = "".to_string();
                    for candidate in gem_response.candidates {
                        let mut message = "".to_string();
                        for part in candidate.content.parts {
                            if let Some(text) = part.text {
                                message += &text;
                            }
                        }
                        candidate_total_response += &message;
                    }

                    candidate_total_response
                } else {
                    println!("Generated with no content in Gemini Response");
                    return;
                }
            );
            println!("Done with generation!");
            res_str
        }
        Err(api_error) => {
            println!("Error with generation! {:#?}", api_error);
            "Error with generation".to_string()
        }
    };

    CONVERSATION_MUTEX
        .lock()
        .await
        .push(response_str.to_string());
}