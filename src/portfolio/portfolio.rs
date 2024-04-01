// // use rocket::*;
// // use rocket::response::content::RawHtml;

// // Reused Components
// pub fn nav_bar() -> rocket::response::content::RawHtml<& 'static str> {
//     rocket::response::content::RawHtml(r#"
//         <nav class='navbar navbar-expand-lg fixed-top bg-white navbar-dark portfolio-navbar gradient' style='background:linear-gradient(-53deg, var(--bs-teal), var(--bs-cyan)), var(--bs-cyan);'>
//             <div class='container'><a class='navbar-brand logo' href='index.html'>John Paul Bunn</a>
//                 <button data-bs-toggle='collapse' data-bs-target='#navbarNav' class='navbar-toggler'>
//                     <span class='visually-hidden'>Toggle navigation</span>
//                     <span class='navbar-toggler-icon'></span>
//                 </button>
//                 <div class='collapse navbar-collapse' id='navbarNav'>
//                     <ul class='navbar-nav ms-auto'>
//                         <li class='nav-item'><a class='nav-link' href='index.html'>Home</a></li>
//                         <li class='nav-item'><a class='nav-link active' href='aboutme.html'>About Me</a></li>
//                         <li class='nav-item'><a class='nav-link' href='cv.html'>CV</a></li>
//                         <li class='nav-item'><a class='nav-link' href='projects-grid-cards.html'>Projects</a></li>
//                         <li class='nav-item'><a class='nav-link' href='contact-me.html'>Contact Me</a></li>
//                         <li class='nav-item'><a class='nav-link' href='project-page.html'>About This Site</a></li>
//                     </ul>
//                 </div>
//             </div>
//         </nav>
//         "#)
// }

// /*
// #[component]
// pub fn Footer() -> impl IntoView {
//     view! {
//         <footer class="page-footer'>
//             <div class="container'>
//                 <div class="links'>
//                     <a href="aboutme.html'><u>About me</u></a>
//                     <a href="contact-me.html'><u>Contact me</u></a>
//                     <a href="projects-grid-cards.html'><u>My projects</u></a>
//                 </div>
//                 <div class="social-icons">
//                     <a href="https://www.facebook.com/profile.php?id=100009795074402"><i class="icon ion-social-facebook"></i></a>
//                     <a href="https://www.instagram.com/jp.bunn/"><i class="icon ion-social-instagram-outline"></i></a>
//                     <a href="https://www.linkedin.com/in/jpbunn/"><i class="icon ion-social-linkedin"></i></a>
//                 </div>
//                 <div>
//                     <p> "©" 2024 John Paul Bunn<br/></p>
//                 </div>
//             </div>
//         </footer>
//         // <script src="assets/bootstrap/js/bootstrap.min.js"></script>
//         // <script src="https://cdnjs.cloudflare.com/ajax/libs/pikaday/1.6.1/pikaday.min.js"></script>
//         // <script src="assets/js/script.min.js"></script>
//     }
// }

// // Page Components
// #[component]
// fn HomePage() -> impl IntoView {
//     view! {
//         <main class="page lanidng-page">
//             <section class="portfolio-block block-intro">
//                 <div class="container">
//                     <div class="avatar" id="avatar_business"></div>
//                     <div class="about-me">
//                         <p>John Paul Bunn<br />B.S Computer Science & Computer Engineering<br/><br/>Coffee Crusader,
//                             Practicing Cyberdemon, Occasional Word-Sorcerer</p><a class="btn btn-outline-primary"
//                             role="button" href="contact-me.html">Contact me</a>
//                     </div>
//                 </div>
//             </section>
//             <section class="portfolio-block photography"></section>
//             <section class="portfolio-block call-to-action border-bottom">
//                 <div class="container">
//                     <div class="d-flex justify-content-center align-items-center content">
//                         <p style="font-size:22px;text-align:center;">Engineer | Writer | Community Leader<br/><br/>I always
//                             have a story to tell. Learn more about me below.</p>
//                     </div>
//                 </div>
//             </section>
//             <section class="portfolio-block skills">
//                 <div class="container">
//                     <div class="row">
//                         <div class="col-md-4"><a href="aboutme.html" style="color:rgb(33,37,41);">
//                                 <div class="card border-0 special-skill-item">
//                                     <div class="card-header bg-transparent border-0"><i
//                                             class="icon ion-android-contact"></i></div>
//                                     <div class="card-body">
//                                         <h3 class="card-title"><u>Personable</u></h3>
//                                         <p class="card-text">Learn about who I am, behind the engineering. Read about my
//                                             hobbies, events, and my work in my community!</p>
//                                     </div>
//                                 </div>
//                             </a></div>
//                         <div class="col-md-4"><a href="cv.html" style="color:rgb(33,37,41);">
//                                 <div class="card border-0 special-skill-item">
//                                     <div class="card-header bg-transparent border-0"><i class="icon ion-filing"></i></div>
//                                     <div class="card-body">
//                                         <h3 class="card-title"><u>Professional</u></h3>
//                                         <p class="card-text">Making an impact in the world. Read about my experience at
//                                             Google, Direct Supply, and beyond.</p>
//                                     </div>
//                                 </div>
//                             </a></div>
//                         <div class="col-md-4"><a href="projects-grid-cards.html" style="color:rgb(33,37,41);">
//                                 <div class="card border-0 special-skill-item">
//                                     <div class="card-header bg-transparent border-0"><i
//                                             class="icon ion-ios-gear-outline"></i></div>
//                                     <div class="card-body">
//                                         <h3 class="card-title"><u>Projects</u></h3>
//                                         <p class="card-text">"Building up the world around me. Read about how I am keeping
//                                             myself busy with projects."</p>
//                                     </div>
//                                 </div>
//                             </a></div>
//                     </div>
//                 </div>
//             </section>
//         </main>
//     }
// }

// #[component]
// fn AboutMePage() -> impl IntoView {
//     view! {
//         <main class="page lanidng-page"><section class="portfolio-block block-intro"><div class="container"><div class="row"><div class="col"><div class="avatar" id="avatar_casual"></div></div><div class="col"><div class="about-me"><p>I believe you can learn a lot about a person by their actions. <br/>Here is what I have been up to.</p><a class="btn btn-outline-primary" role="button" href="contact-me.html">Contact me</a></div></div></div></div></section><section class="portfolio-block skills" style="padding:50px;"><div class="container"><div class="heading"><h2>Beyond the tech</h2><div><h3><strong>Writing</strong></h3><p>I am a fiction writer! I have written several short-stories, some of which have won short-story competitions. I am actively working on novel and podcast projects.<br/><br/>Feel free to ask me about my publications online.</p></div><div><h3><strong>Escape Rooms</strong></h3><p>I have been in multiple escape teams and organized an "Escape Room League" while I was with Google in 2023. I have a 95% success rate. Once I am finished with university I plan on competing for best times at escape rooms in my area.</p></div><div><h3><strong>Tabletop</strong></h3><p>While in university, I was the president of a club called MAGE, which was a "SciFi, Anime, and Gaming" club on campus; essentially an everything-club for all things geek. I am also a playtester for multiple board games during their development and experimental phases.<br/><br/>While at university, I have been told I am the best game-master that my TTRPG players have ever had for every game I have hosted. While I am confident I am not world-class, my writing background has led to games that are far-more thrilling than top-TTRPG channels online.</p></div><div><h3><strong>eSports</strong></h3><p>During the Fall Semester I was invited to join the Overwatch eSports team at my university, where we regularly competed and took 3rd place regionally against other university teams within our division.</p></div></div></div></section></main>
//     }
// }

// #[component]
// fn CVPage() -> impl IntoView {
//     view! {
//         <main class="page cv-page"><section class="portfolio-block block-intro border-bottom"><div class="container"><div class="row"><div class="col"><div class="avatar" id="avatar_business"></div></div><div class="col"><div class="about-me"><p>Computer Scientist. Computer Engineer. Software Engineer. Embedded Programmer. Hardware Tester.<br/>Data Scientist.</p><a class="btn btn-outline-primary" role="button" href="contact-me.html">Contact me</a></div></div></div></div></section><section class="portfolio-block cv"><div class="container"><div class="work-experience group"><div class="heading"><h2 class="text-center">Summary</h2><p>Dual-bachelors in Computer Science and Computer Engineering. Experience in full-stack application development, distributed systems, test-suite management and continuous integration, embedded systems, and data science.<br/><br/><strong>Programming:</strong> Java, Python, "C/C++/C#", Go, Rust, HTML/CSS, JavaScript, TypeScript, MATLAB, VHDL/Verilog, ARM Assembly<br/><br/><strong>Tooling: </strong>Git, Jupyter Notebooks, AWS Glue and Batch scripts, Unity, React.JS, Tensorflow, Pytorch, Numpy/Pandas, Kicad, Onshape<br/><br/><strong>Management:</strong>AGILE/SCRUM, Github/Gitlab, Jira/Conflux, Notion<br/><br/><strong>Experience Domains:</strong><br/>Full Stack<br/>AI/ML Development<br/>Data Science<br/>Low-level/Embedded Development<br/>Compiler Optimization<br/>Distributed Systems Programming<br/>Product & Solution Development<br/>Hardware Testing<br/>Continuous Integration<br/>Site Reliability<br/>PCB design<br/>Linux Development</p></div></div><div class="work-experience group"><div class="heading"><h2 class="text-center">Work Experience</h2></div><div class="item"><div class="row"><div class="col-md-6"><h3>Software Engineer Intern</h3><h4 class="organization">Google, inc</h4></div><div class="col-md-6"><span class="period">May 2023 - August 2023</span></div></div><p class="text-muted">- Implemented Python logic to parse 20 configuration files and collect data across the Google codebase.<br/>- Developed multiple SQL scripts to compile resource usage data of four internal tool sets used by over 3000 systems.<br/> - "Designed a dashboard for illustrating resource usage and adoption of new tool suites for use by Google Maps resource managers and team leaders."<br/>- Built a test suite in Golang for validating that new systems were correctly onboarding new tooling.<br/>- Volunteered as an embedded support assistant for research on an internal recycled computing project.<br/>- Continued assisting intern events across the Bay Area including starting an intern escape room league, game nights, and more.</p></div><div class="item"><div class="row"><div class="col-md-6"><h3>Data Science & Software Engineer Intern</h3><h4 class="organization">ChemDirect</h4></div><div class="col-md-6"><span class="period">Nov 2022 - May 2023</span></div></div><p class="text-muted">- Developed data cleansing and UI solutions when suppliers provide data in Node JS and React<br/>- Supported data analysis with AWS Glue and Jupyter Notebooks using Numpy/Pandas<br/>- Developed applications using Large Language Models to determine product density, similarity analysis, and more</p></div><div class="item"><div class="row"><div class="col-md-6"><h3>Software Engineer Intern</h3><h4 class="organization">Google, inc</h4></div><div class="col-md-6"><span class="period">May 2022 - August 2022</span></div></div><p class="text-muted">- Implemented and launched a new distributed production service in Golang to coordinate stateful probing of Google Mobile Maps infrastructure.<br/>- Implemented build logic in Bazel to adapt 100+ Android UI Orchestration Tests for running continuously against multiple live and pre-production backend services.<br/>- Volunteered as a Teaching Assistant for the Google Computer Science Summer Institute teaching beginner to intermediate JavaScript, Data Structures, Bulma, HTML and CSS to over 50 students.<br/>- Coordinated and assisted with intern events across the Bay Area including starting an intern bowling league, board game nights, and more.</p></div><div class="item"><div class="row"><div class="col-md-6"><h3>Embedded Software Engineer Intern</h3><h4 class="organization">Astronautics Corporation of America</h4></div><div class="col-md-6"><span class="period">Nov 2021 - May 2022</span></div></div><p class="text-muted">- Tested and verified the functionality of FPGA devices<br/>- Designed and optimized YOCTO embedded applications for wireless communication systems using Python and C<br/>- Created Python scripts for generating descriptions of embedded programs for security purposes</p></div><div class="item"><div class="row"><div class="col-md-6"><h3>STEPIntern</h3><h4 class="organization">Google, inc</h4></div><div class="col-md-6"><span class="period">May 2021 - Summer 2022</span></div></div><p class="text-muted">- Completed a challenging technical project consisting of a Java bytecode optimizer used on all Google apps with a compiler using C++, performing static-analysis with Single-Static-Assignment (SSA) and Intermediate Representation (IR)<br/>- Built optimizations to resolve contradicting comparison statements in Java, as well as optimizations for unboxing wrapper-classes and Java Optionals by manipulating the byte code, reducing the size of major Google apps by several K<br/>- Attended technical talks by Googlers, gaining insight into technical interview preparation, learning about "Google's" coding practices and technologies, and developing other skills to set up for success<br/>- Received coaching and mentorship from Google engineers<br/>- Built personal networks and friendships with a diverse group of students who share a passion for technology</p></div><div class="item"><div class="row"><div class="col-6"><h3>Software Engineer Intern</h3><h4 class="organization">Direct Supply</h4></div><div class="col-md-6"><span class="period">Sep 2020 - May 2021</span></div></div><p class="text-muted">- Created an ASP.NET application for uploading files, downloading files, and making new directories using the Google Drive API<br/>- Worked on a project management application in full-stack development using React.JS , TypeScript, "C#/.NET", and Postgres SQL.<br/>- Developed components for a Windows Forms application for products<br/><br/></p></div><div class="item"><div class="row"><div class="col-md-6"><h3>Game Developer Apprentice</h3><h4 class="organization">University of Wisconsin Stevens Point</h4></div><div class="col-md-6"><span class="period">Jan 2020 - Aug 2020</span></div></div><p class="text-muted">- Collaborated with a team of developers to research methods of deploying accessible and educational gameplay using "C#" and Unity<br/>- Winner of the 2020 WiSys Quick Pitch UW- Stevens Point competition<br/></p></div></div><div class="education group"><div class="heading"><h2 class="text-center">Education</h2></div><div class="item"><div class="row"><div class="col-md-6"><h3>Computer Science & Computer Engineering, minor in Mathematics</h3><h4 class="organization">Milwaukee School of Engineering</h4></div><div class="col-md-6"><span class="period">09/2020 - 05/2024</span></div></div><p class="text-muted"><strong>GPA</strong>: 3.45<br/><strong>Honors/Awards</strong>: Deans List, High Honors<br/><strong>Scholarships</strong>:Destination MSOE, Fred Loock, Grohmann Scholars, Scott & Kathy Weaver, UMSA Security<br/><strong>Relevant Coursework</strong>:Algorithms & Advanced Data Structures, Circuits 1-3, Computer Architecture, Computer Networking, Data Science, Data Structures & Algorithms, Database Systems, Deep Learning, Digital Logic 1-2, Digital Signal Processing, Discrete Mathematics, Embedded Systems 1-3, Embedded Systems Fabrication, Information Security, Machine Learning, Operating Systems, Physics 1-3, Physics of Electromagnetic Materials and Semiconductors, Partial Differential Equations, Probability & Statistics, Signals & Systems, Software Component Design, Software Development 1-2, Software Engineering Process<br/><br/></p></div></div><div class="education group"><div class="heading"><h2 class="text-center">Leadership & Volunteering</h2></div><div class="item"><div class="row"><div class="col-md-6"><h3>Convention Lead - Concinnity 24</h3></div><div class="col-md-6"><span class="period">09/2024- 05/2024</span></div></div><p class="text-muted">Led the Concinnity Planning Committee (team of over 20 members, students and non-students) to coordinate the Concinnity 24 annual Sci-Fi, Anime, and Gaming convention, including leading weekly team planning meetings, designing posters, drafting marketing strategies, growing student involvement, and more. Through this leadership, we secured contracts with multiple sponsors and special guests (including SungWon Cho, A.K.A "ProZD"), media coverage, and many vendors.</p></div><div class="item"><div class="row"><div class="col-md-6"><h3>President - Society of Software Engineers</h3></div><div class="col-md-6"><span class="period">01/2023 - 03/2024</span></div></div><p class="text-muted">Organized professional events including introductory programming, workshops, speaker events, company tours, and more for over 200 students, as well as worked with a team of 6 eBoard members to streamline executive processes for sub-clubs so they could remain focused on their main agendas including competitive programming, website development, embedded systems development, and game development.<br/></p></div><div class="item"><div class="row"><div class="col-md-6 col-lg-6"><h3>Vice President & Interim President - MAGE</h3></div><div class="col-md-6" style="width: 368px;"><span class="period">03/2023- 05/2024</span></div></div><p class="text-muted">Became interim president in February of 2024 and led many events including escape room and movie outings, large gaming events, and facilitating regular MAGE meetups including chess, film, board gaming, card-gaming, tabletop-RPG, and anime meetups. Also worked with a team of 12 eBoard members to manage club spaces/property, increase club membership (over 400 students), and liaison with university administration.</p></div></div><div></div><div class="group"><div class="contact-info portfolio-info-card"><h2>Contact Info</h2><div class="row"><div class="col-1"><i class="icon ion-person"></i></div><div class="col-9"><span>John Paul Bunn</span></div></div><div class="row"><div class="col-1"><i class="icon ion-ios-telephone"></i></div><div class="col-9"><span>+715 303 9118</span></div></div><div class="row"><div class="col-1"><i class="icon ion-at"></i></div><div class="col-9"><span>JohnPaulBunn@gmail.com</span></div></div></div></div></div></section></main>
//     }
// }

// #[component]
// fn AboutThisPortfolioPage() -> impl IntoView {
//     view! {
//         <main class="page project-page"><section class="portfolio-block project"><div class="container"><div class="heading"><h2>About this portfolio</h2></div><div class="image" id="design_rationale"></div><div class="row"><div class="col-12 col-md-6 offset-md-1 info"><h3>Design Rationale</h3><p style="font-size:20px;">The primary goal of this portfolio was to truly represent who I am. Sure, you could show only who you are from a technical perspective, but if you knew more about my life --what I love, and who I strive to be-- then when that "'perfect match'" is made with a potential employer I know I will be able to make that impact on the world. Not only would I have a fulfilling career, but finally be able to make that difference in the world. "That's" where the primary influence in the design originated: why I chose to include my writing, gaming, and more aspects about my life and who I am beyond the resume. <br/><br/></p><p></p></div><div class="col-12 col-md-3 offset-md-1 meta"><h3 class="text-muted"><br/>"We are dreamers, shapers, singers, and makers. We study the mysteries of laser and circuit, crystal and scanner, holographic demons and invocations of equations. These are the tools we employ, and we know many things."<br/>~Elric, Babylon 5<br/><br/></h3></div></div></div></section></main>
//     }
// }

// #[component]
// fn ProjectPage() -> impl IntoView {
//     view! {
//         <main class="page projects-page">
//             <section class="portfolio-block projects-cards">
//                 <div class="container">
//                     <div class="heading">
//                         <h2>Recent Projects</h2>
//                     </div>
//                 </div>
//                 <div class="container">
//                     <div class="row">
//                         <div class="col-md-4">
//                             <h2 style="text-align: center;"><strong>"BevyPong - Game Development using Bevy and Rust"</strong></h2>
//                         </div>
//                         <div class="col-md-4">
//                             <h2 style="text-align: center;"><strong>"Networking Node Interface on STM32 using Rust Embassy"</strong></h2>
//                         </div>
//                         <div class="col">
//                             <h2 style="text-align: center;"><strong>"Random Forest Model for Matching Board Game Weights to Player Preferences in NumPy/Pandas"</strong></h2>
//                         </div>
//                     </div>
//                 </div>
//                 <div class="container">
//                     <div class="row">
//                         <div class="col-md-4">
//                             <p class="text-center">"Interactive 2D video-game using Bevy, a Rust Game Engine, that implements
//                                 aspects of the Entity-Component-System." <br /> <br /> "Implemented player and ball movement
//                                 systems with collision detection and physics." <br /> <br /> "Developed systems for player and
//                                 ball movements, handling collisions with walls, the player paddle, and the ceiling." <br />
//                                 "Used timers to manage game ticks and ensure smooth gameplay." <br /> <br /> "Integrated with
//                                 Bevy's input system to handle keyboard input for player movement." <br /> <br /> "Organized code
//                                 into modules, systems, and resources for maintainability and readability." <br /> <br />
//                                 "Leveraged Bevy's built-in components and resources for rendering and managing game entities."
//                             </p>
//                         </div>
//                         <div class="col-md-4">
//                             <p class="text-center">"Implemented a network-communication node intended to communicate with a
//                                 hub-device with other classmates' network nodes".<br /><br />"Utilized the Embassy Executor
//                                 framework to manage asynchronous tasks for receiving, monitoring, and transmitting messages,
//                                 ensuring non-blocking operation and efficient resource utilization during
//                                 interrupts."<br /><br />"Leveraged mutexes for thread-safe access to shared uses, including
//                                 access to a custom message-building data structure."<br /><br />"Integrated manchester encoding,
//                                 header and trailer encode/decoding (including CRC calculation and
//                                 verification)."<br /><br />"Implemented custom transmitter and receiver with random-backoff
//                                 along communication lines as well as a state-observer to detect/avoid data
//                                 collisions."<br /><br />"Created asynchronous tests using the embedded-test rust-crate, as well
//                                 as designing hardware simulations using Wokwi."</p>
//                         </div>
//                         <div class="col-md-4">
//                             <p class="text-center">"Analyzed the relationship between board game weight and many data points
//                                 from the Board Game Geek dataset, including a game's predetermined complexity, rating, number of
//                                 players, and number of owners, and theme of the game."<br /><br /> "Related game weight to
//                                 cognitive load, as well as relating the age of a board game to their unique
//                                 complexity."<br /><br />"Performed feature analysis on the attributes to the independent
//                                 variable to determine their importance in determining game weight."<br /><br />"Determined
//                                 statistical significance through Linear Regression and Kruskal-Wallis tests."</p>
//                         </div>
//                     </div>
//                 </div>
//             </section>
//             <section class="portfolio-block projects-cards">
//                 <div class="container">
//                     <div class="row">
//                         <div class="col-md-4">
//                             <h2 style="text-align: center;"><strong>"Interconnected Tennis Scoreboards"</strong></h2>
//                         </div>
//                         <div class="col-md-4">
//                             <h2 style="text-align: center;"><strong>"Classifying Objects in Underwater Sonar Images"</strong></h2>
//                         </div>
//                         <div class="col">
//                             <h2 style="text-align: center;"><strong>"Custom WiFi Clock - Custom PCB Fabrication and 3D Modeling"</strong></h2>
//                         </div>
//                     </div>
//                 </div>
//                 <div class="container">
//                     <div class="row">
//                         <div class="col-md-4">
//                             <p class="text-center">"Integrated the Arduino Painless Mesh library to interconnect 7 ESP32s for
//                                 sharing the state of multiple tennis-courts across a mesh network."<br /><br />"Used SPIFF to
//                                 maintain data-files on flash memory, maintaining a history, scoreboard status, and more even
//                                 when the device is shut down.<br /><br />Implemented a local WiFi network hosted on multiple
//                                 ESP32s to view or change the state of the tennis scoreboards on a web page supporting
//                                 asynchronous updates with HTTP POST and GET requests."
//                             </p>
//                         </div>
//                         <div class="col-md-4">
//                             <p class="text-center">"Developed a Convolutional Neural Network (CNN) model based on YOLOv4
//                                 architecture for underwater object detection and classification using MATLAB Deep Learning
//                                 Toolbox and Computer Vision Toolbox, achieving high accuracy and
//                                 precision."<br /><br />"Utilized data preprocessing techniques including data cleansing on a
//                                 dataset of 7600 black and white sonar images using geometric transformations, logarithmic
//                                 compression, and denoising encoding."<br /><br />"Collaborated with a team to develop a unique
//                                 approach for processing multibeam sonar images using deep learning techniques, resulting in a
//                                 model capable of detecting and classifying objects with high accuracy in underwater
//                                 environments."
//                             </p>
//                         </div>
//                         <div class="col-md-4">
//                             <p class="text-center">"Designed and developed a printed circuit board (PCB) schematic using KiCad,
//                                 ensuring precise layout and functionality for electronic systems, including routing, tracing,
//                                 and handling attenuation."<br /><br />"Manually placed and soldered individual-components onto
//                                 the fabricated PCB."<br /><br />"Configured the clock with custom embedded code in Arduino C,
//                                 emulating the ESP32 communications protocol, including implementations for wifi-connectivity,
//                                 polling weather data, and displaying custom messages"<br /><br />"Created a 3D model in Onshape
//                                 for the clock's frame and custom buttons."
//                             </p>
//                         </div>
//                     </div>
//                 </div>
//             </section>
//             <section class="portfolio-block projects-cards">
//                 <div class="container">
//                     <div class="row">
//                         <div class="col-md-4">
//                             <a href="https://covid19.spaceappschallenge.org/challenges/covid-challenges/integrated-assessment/teams/hexhax/project">
//                                 <img class="card-img-top scale-on-hover" src="tech/map.jpg" alt="Card Image" height="200px" />
//                             </a>
//                         </div>
//                         <div class="col-md-4">
//                             <a href="https://devpost.com/software/bayhax">
//                                 <img class="card-img-top scale-on-hover" src="tech/bear.jpg" alt="Card Image" height="200px" />
//                             </a>
//                         </div>
//                         <div class="col-md-4">
//                             <img class="card-img-top scale-on-hover" src="tech/cat.jpg" alt="Card Image" height="200px" />
//                         </div>
//                     </div>
//                 </div>
//                 <div class="container">
//                     <div class="row">
//                         <div class="col-md-4"><a
//                                 href="https://covid19.spaceappschallenge.org/challenges/covid-challenges/integrated-assessment/teams/hexhax/project"
//                                 style="color:var(--bs-body-color);">
//                                 <h3 class="text-center" style="text-decoration:underline;">"Artificial Intelligence Derived
//                                     Predictive Analysis of COVID-19 over Time (AID PACT)"</h3>
//                             </a></div>
//                         <div class="col-md-4"><a href="https://devpost.com/software/bayhax">
//                                 <h3 class="text-center" style="color:var(--bs-body-color);text-decoration:underline;">BayHax
//                                 </h3>
//                             </a></div>
//                         <div class="col-md-4">
//                             <h3 class="text-center">"Borb's Big Adventure"</h3>
//                         </div>
//                     </div>
//                 </div>
//                 <div class="container">
//                     <div class="row">
//                         <div class="col-md-4">
//                             <p class="text-center"><strong>Summary</strong><br />"Our interactive map displays current and
//                                 AI-predicted future COVID-19 cases and unemployment rates over time. Using a Bayesian Ridge
//                                 Regression, we trained our machine learning model by integrating Earth Observation derived
//                                 features and socioeconomic data. By providing these location-based predictions, we hope to
//                                 help the public make better-informed decisions regarding safety."<br />"My involvement was in
//                                 the depiction and use of the data with our interactive tool, integrating with Google Maps.
//                                 Learn more at the project site!"<br /></p>
//                         </div>
//                         <div class="col-md-4">
//                             <p class="text-center"><strong>Summary</strong><br />"Winning 2nd place during the 2020
//                                 NotUniversity Hackathon, BayHax is an emotional health tracking teddy bear, fitted with a
//                                 Raspberry Pi, speakers, and buttons. Working as a team of 5 over the course of a weekend
//                                 during this competitive event, we successfully developed a working prototype, as well as an
//                                 application visualizing the data collected by BayHax. My involvement was to develop the
//                                 infrastructure between BayHax and the application and graphing the data using the Chart.js
//                                 library. BayHax's data, which pertained to emotional states such as anger, sadness, and
//                                 joy, were charted and cross-referenced with time and interactions with the buttons. While
//                                 developed during a hackathon, BayHax's primary focus was to help parents to better
//                                 understand the emotional state of their children, allowing them to track times and dates
//                                 when their child might be experiencing a particular emotional state".<br /></p>
//                         </div>
//                         <div class="col-md-4">
//                             <p class="text-center"><strong>Summary</strong><br />"Borb's Big Adventure" is a reflexive
//                                 response and physics-based game played as a side-scrolling adventure. Developed during the
//                                 game development research project under Dr. T. Krause at the University of Wisconsin Stevens
//                                 Point, "Borb's Big Adventure" was meant to help children learn more about momentum and
//                                 kinematic forces by playing as Borb, the ball-orb-round-marble-cat. My task was to develop
//                                 the grappling movement mechanism, which was used as a way for the player to manipulate a
//                                 rope with tension in order to "'grapple'" to their goal. After 6 months of development,
//                                 "Borb's Big Adventure" was demoed at the 2020 WiSys Quick Pitch UW- Stevens Point
//                                 competition where it earned a research grant to continue the project. One of the
//                                 biggest challenges of this project was the many various aspects the project took on: for
//                                 example, there was a significant amount of visual design, simulated kinematics versus actual
//                                 kinematics, and more. "Borb's Big Adventure" was developed in the Unity Game Engine using
//                                 "C#", and developed collectively by "Dr. Krause's" research team.<br /></p>
//                         </div>
//                     </div>
//                 </div>
//                 <div class="container">
//                     <div class="row">
//                         <div class="col-md-4">
//                             <p class="text-center"><strong>Analysis</strong><br />Our predictive model integrates Earth
//                                 Observation-derived features like population density and global human modification with
//                                 socio-economic data including city accessibility, Gini coefficient (the measurement of
//                                 wealth inequality), and GDP (gross domestic product) to predict future cases of COVID-19
//                                 through machine learning.<br /><br />By mapping current and future cases on an international
//                                 scale, this application allows global health agencies to distribute personal protective
//                                 equipment (PPE), ventilators, gurneys, and other necessary medical resources to states and
//                                 countries that experience a greater effect of COVID-19.Our AI predictions of future
//                                 cases allow government leaders to make better-informed decisions about lifting or extending
//                                 lockdown and social distancing regulations. Informing the public about potential spread will
//                                 help ensure that they are protecting themselves appropriately. Concurrently, predictions of
//                                 high numbers of future cases may encourage lawmakers to consider implementing or increasing
//                                 stimulus checks and other forms of relief.<br /><br /></p>
//                         </div>
//                         <div class="col-md-4">
//                             <p class="text-center"><strong>Analysis</strong><br />By recognizing trends collected by an AI
//                                 facial recognition algorithm, particular determinations could be reliably made about "one's"
//                                 emotional state at a certain date and time. What I treasure most about this project was
//                                 picking up a new technology and employing it quickly to a relevant and impactful
//                                 application. This project also drew me closer to the applications of artificial intelligence
//                                 in hardware projects; the possibility to apply collected data to effectively predict
//                                 emotional responses was particularly interesting to me. Working on BayHax was also one of my
//                                 first and most valuable experiences of working on a technical challenge with a team right
//                                 out of high school. Our team --HexHax-- used our shared experience from Google CSSI in order
//                                 to craft a solution that rocked the leaderboards. If I could, I would take more
//                                 responsibility in the project itself, and take on more of a leadership role.<br /><br /></p>
//                         </div>
//                         <div class="col-md-4">
//                             <p class="text-center"><strong>Analysis</strong><br />Incorporating physics into a virtual
//                                 environment (as a 2D video game) was a difficult feat; one that involved a significant
//                                 amount of debugging and refining. As a programmer on the team, I was charged with the
//                                 movement mechanics of the game, including jumping, walking or rolling, and some work on
//                                 grappling. This project became one of the best experiences I ever had in resolving
//                                 algorithmic solutions to complicated situations; such as where a tether should render, and
//                                 which direction the centripetal acceleration of a bouncing circular kitten should be, which
//                                 inspired me to pursue increasingly complicated and more impactful projects to become
//                                 involved in.<br /><br /></p>
//                         </div>
//                     </div>
//                 </div>
//             </section>
//         </main>
//     }
// }

// #[component]
// fn ErrorPage() -> impl IntoView {
//     let params = use_params_map();
//     let p_unknown = move || params.with(|p| p.get("else").cloned().unwrap_or_default());

//     let unknown = p_unknown();

//     view! {
//         <main class=format!("h-screen w-full flex flex-col items-center justify-center font-robotomono")>
//             <p class="">Unknown command: {unknown}</p>
//         </main>
//     }
// }
// */