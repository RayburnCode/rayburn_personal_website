use dioxus::prelude::*;

#[component]
pub fn Resume() -> Element {
    rsx! {
        div { class: "max-w-4xl mx-auto p-6 text-gray-800 dark:text-gray-200",
            // Experience Section
            section { class: "mb-8",
                h2 { class: "text-2xl font-bold mb-4 border-b border-gray-300 dark:border-gray-600 pb-2", "EXPERIENCE" }
                
    // Everyone's Processing Company 
    div { class: "mb-6",
        h4 { class: "text-xl font-semibold", "Everyone's Processing Company, Glendale, CA - Mortgage Loan Processor" }
        p { class: "italic text-gray-600 dark:text-gray-400 mb-2", "April 2024 - Present" }
        ul { class: "list-disc pl-6 space-y-1",
            li { "Review and process residential mortgage loan applications, ensuring strict compliance with lender guidelines and regulatory requirements throughout the entire loan lifecycle from submission to final approval." }
            li { "Coordinate between multiple stakeholders including borrowers, loan officers, underwriters, and third-party vendors to collect and verify financial documentation, order property appraisals, and resolve file discrepancies." }
            li { "Maintain organized loan files and documentation while meeting critical deadlines in a fast-paced lending environment." }
        }
    }

                // California Bank & Trust
                div { class: "mb-6",
                    h4 { class: "text-xl font-semibold", "California Bank & Trust, San Diego, CA - Credit Analyst" }
                    p { class: "italic text-gray-600 dark:text-gray-400 mb-2", "July 2022 - Present" }
                    ul { class: "list-disc pl-6 space-y-1",
                        li { "Collaborated closely with Regional Managers to address any covenant-related issues, providing guidance and assistance in finding appropriate solutions." }
                        li { "Applied financial analysis techniques to examine financial statements, related materials, and market conditions to determine the viability of loan applications." }
                        li { "Utilized appropriate tax analysis methods to assess cash flow and financial stability of applicants." }
                    }
                }
                
                // Global Equity Finance
                div { class: "mb-6",
                    h4 { class: "text-xl font-semibold", "Global Equity Finance, San Diego, CA - Various roles" }
                    p { class: "italic text-gray-600 dark:text-gray-400 mb-2", "May 2020 - July 2022" }
                    
                    ol { class: "list-decimal pl-6 space-y-4",
                        li {
                            h5 { class: "font-semibold", "Senior Credit Analyst" }
                            ul { class: "list-disc pl-6 mt-1 space-y-1",
                                li { "Worked with loan officers and processors to ensure loan file moves from initial submission through funding." }
                                li { "Review and analyze financial statements, tax returns and verify borrower's information to assess eligibility of loan programs." }
                                li { "Trained new Credit Analysts on new programs such as HELOCs." }
                            }
                        }
                        
                        li {
                            h5 { class: "font-semibold", "Mortgage Loan Originator" }
                            ul { class: "list-disc pl-6 mt-1 space-y-1",
                                li { "Assist clients with initial loan applications that directly optimize their financial needs." }
                                li { "Utilize knowledge of Lender Overlays and Guidelines to assure loans are tailored to fit client needs." }
                            }
                        }
                    }
                }
                
                // Lincoln Finance Company
                div { class: "mb-6",
                    h4 { class: "text-xl font-semibold", "Lincoln Finance Company, San Diego, CA — Collections Administrator" }
                    p { class: "italic text-gray-600 dark:text-gray-400 mb-2", "June 2018 - May 2020" }
                    ul { class: "list-disc pl-6 space-y-1",
                        li { "Maintenance and upkeep with commercial line of credit. Tasks including leading preparation for audits, producing monthly financial reports, and preparing borrowing base reporting. Creating Excel Macros for efficiency." }
                        li { "In charge of partnering with new companies to streamline collections processes as well as maintaining healthy business relations with the companies." }
                    }
                }
            }
            
            hr { class: "my-6 border-gray-300 dark:border-gray-600" }
            
            // Education Section
            section { class: "mb-8",
                h2 { class: "text-2xl font-bold mb-4 border-b border-gray-300 dark:border-gray-600 pb-2", "EDUCATION" }
                
                div { class: "mb-4",
                    h5 { class: "text-lg font-semibold", "San Diego State University, San Diego, CA — Bachelor Degree in Statistics – Emphasis in Actuarial Science" }
                    p { class: "italic text-gray-600 dark:text-gray-400", "August 2016 - December 2018" }
                }
                
                div {
                    h5 { class: "text-lg font-semibold", "Grossmont Community College, El Cajon, CA — Associate's Degree in Mathematics" }
                    p { class: "italic text-gray-600 dark:text-gray-400", "September 2013 - June 2016" }
                }
            }
            
            hr { class: "my-6 border-gray-300 dark:border-gray-600" }
            
            // Skills Section
            section {
                h2 { class: "text-2xl font-bold mb-4 border-b border-gray-300 dark:border-gray-600 pb-2", "SKILLS" }
                
                ul { class: "list-disc pl-6 space-y-2",
                    li { 
                        "Notary Public/ Signing Agent"
                        ul { class: "list-[circle] pl-6",
                            li { "# 2333116" }
                        }
                    }
                    li { "NMLS Lic #2173974" }
                    li { "FNMA/FHLMC/FHA/VA/Non-QM Guidelines" }
                    li { "AUS, DU/LP" }
                    li { "Verifying income from Personal and Business Tax Returns" }
                }
            }
        }
    }
}