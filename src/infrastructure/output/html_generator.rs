use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::presentation::dto::AnalysisOutput;

pub struct HtmlGenerator;

impl HtmlGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate<P: AsRef<Path>>(&self, output: &AnalysisOutput, path: P) -> Result<()> {
        let html = self.generate_string(output)?;
        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;
        Ok(())
    }

    pub fn generate_string(&self, output: &AnalysisOutput) -> Result<String> {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("    <title>Control-M Migration Analysis</title>\n");
        html.push_str("    <link rel=\"stylesheet\" href=\"https://cdn.datatables.net/1.13.7/css/jquery.dataTables.min.css\">\n");
        html.push_str("    <script src=\"https://code.jquery.com/jquery-3.7.0.min.js\"></script>\n");
        html.push_str("    <script src=\"https://cdn.datatables.net/1.13.7/js/jquery.dataTables.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str(Self::get_css());
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str("    <div class=\"container\">\n");
        html.push_str("        <h1>📊 Control-M to Airflow Migration Analysis</h1>\n");

        // Statistics Cards
        let easy_count = output.jobs.iter().filter(|j| j.migration_difficulty == "Easy").count();
        let medium_count = output.jobs.iter().filter(|j| j.migration_difficulty == "Medium").count();
        let hard_count = output.jobs.iter().filter(|j| j.migration_difficulty == "Hard").count();
        let critical_count = output.jobs.iter().filter(|j| j.is_critical).count();
        
        html.push_str("        <div class=\"stats-grid\">\n");
        html.push_str(&format!("            <div class=\"stat-card\">\n"));
        html.push_str(&format!("                <div class=\"stat-value\">{}</div>\n", output.summary.total_jobs));
        html.push_str("                <div class=\"stat-label\">Total Jobs</div>\n");
        html.push_str("            </div>\n");
        html.push_str(&format!("            <div class=\"stat-card\">\n"));
        html.push_str(&format!("                <div class=\"stat-value\">{}</div>\n", output.summary.total_folders));
        html.push_str("                <div class=\"stat-label\">Folders</div>\n");
        html.push_str("            </div>\n");
        html.push_str(&format!("            <div class=\"stat-card\">\n"));
        html.push_str(&format!("                <div class=\"stat-value\">{:.1}</div>\n", output.summary.average_complexity_score));
        html.push_str("                <div class=\"stat-label\">Avg Complexity</div>\n");
        html.push_str("            </div>\n");
        html.push_str(&format!("            <div class=\"stat-card\">\n"));
        html.push_str(&format!("                <div class=\"stat-value\">{}</div>\n", output.migration_waves.len()));
        html.push_str("                <div class=\"stat-label\">Migration Waves</div>\n");
        html.push_str("            </div>\n");
        html.push_str("        </div>\n");

        html.push_str("        <div class=\"summary\">\n");
        html.push_str("            <h2>📈 Migration Difficulty Distribution</h2>\n");
        html.push_str("            <div class=\"difficulty-grid\">\n");
        html.push_str(&format!("                <div class=\"difficulty-card easy-bg\">\n"));
        html.push_str(&format!("                    <div class=\"difficulty-count\">{}</div>\n", easy_count));
        html.push_str(&format!("                    <div class=\"difficulty-label\">Easy ({:.1}%)</div>\n", 
            (easy_count as f64 / output.summary.total_jobs as f64) * 100.0));
        html.push_str("                </div>\n");
        html.push_str(&format!("                <div class=\"difficulty-card medium-bg\">\n"));
        html.push_str(&format!("                    <div class=\"difficulty-count\">{}</div>\n", medium_count));
        html.push_str(&format!("                    <div class=\"difficulty-label\">Medium ({:.1}%)</div>\n", 
            (medium_count as f64 / output.summary.total_jobs as f64) * 100.0));
        html.push_str("                </div>\n");
        html.push_str(&format!("                <div class=\"difficulty-card hard-bg\">\n"));
        html.push_str(&format!("                    <div class=\"difficulty-count\">{}</div>\n", hard_count));
        html.push_str(&format!("                    <div class=\"difficulty-label\">Hard ({:.1}%)</div>\n", 
            (hard_count as f64 / output.summary.total_jobs as f64) * 100.0));
        html.push_str("                </div>\n");
        html.push_str(&format!("                <div class=\"difficulty-card critical-bg\">\n"));
        html.push_str(&format!("                    <div class=\"difficulty-count\">{}</div>\n", critical_count));
        html.push_str("                    <div class=\"difficulty-label\">Critical</div>\n");
        html.push_str("                </div>\n");
        html.push_str("            </div>\n");
        html.push_str("        </div>\n");

        html.push_str("        <div class=\"waves\">\n");
        html.push_str("            <h2>🌊 Migration Waves Summary</h2>\n");
        html.push_str("            <div class=\"wave-grid\">\n");
        for wave in &output.migration_waves {
            let wave_jobs: Vec<_> = output.jobs.iter()
                .filter(|j| j.migration_wave == wave.wave)
                .collect();
            let avg_complexity: f64 = if !wave_jobs.is_empty() {
                wave_jobs.iter()
                    .map(|j| j.complexity_score as f64)
                    .sum::<f64>() / wave_jobs.len() as f64
            } else {
                0.0
            };
            
            html.push_str(&format!("            <div class=\"wave-card\">\n"));
            html.push_str(&format!("                <div class=\"wave-header\">Wave {}</div>\n", wave.wave_number));
            html.push_str(&format!("                <div class=\"wave-count\">{} jobs</div>\n", wave.jobs.len()));
            html.push_str(&format!("                <div class=\"wave-complexity\">Avg: {:.1}</div>\n", avg_complexity));
            html.push_str(&format!("                <div class=\"wave-reason\">{}</div>\n", wave.reason));
            html.push_str("            </div>\n");
        }
        html.push_str("            </div>\n");
        html.push_str("        </div>\n");

        html.push_str("        <div class=\"jobs\">\n");
        html.push_str("            <h2>📋 Job Details</h2>\n");
        html.push_str("            <p class=\"table-info\">Interactive table with search, sort, and pagination. Showing all jobs.</p>\n");
        html.push_str("            <table id=\"jobsTable\" class=\"display\">\n");
        html.push_str("                <thead>\n");
        html.push_str("                    <tr>\n");
        html.push_str("                        <th>Job Name</th>\n");
        html.push_str("                        <th>Folder</th>\n");
        html.push_str("                        <th>Wave</th>\n");
        html.push_str("                        <th>Complexity</th>\n");
        html.push_str("                        <th>Difficulty</th>\n");
        html.push_str("                        <th>Priority</th>\n");
        html.push_str("                        <th>Critical</th>\n");
        html.push_str("                        <th>Dependencies</th>\n");
        html.push_str("                        <th>Effort (hrs)</th>\n");
        html.push_str("                    </tr>\n");
        html.push_str("                </thead>\n");
        html.push_str("                <tbody>\n");

        for job in &output.jobs {
            let difficulty_class = match job.migration_difficulty.as_str() {
                "Easy" => "easy",
                "Medium" => "medium",
                "Hard" => "hard",
                _ => "",
            };

            html.push_str("                    <tr>\n");
            html.push_str(&format!("                        <td>{}</td>\n", job.job_name));
            html.push_str(&format!("                        <td>{}</td>\n", job.folder_name));
            html.push_str(&format!("                        <td><span class=\"wave-badge\">Wave {}</span></td>\n", job.migration_wave));
            html.push_str(&format!("                        <td>{}</td>\n", job.complexity_score));
            html.push_str(&format!("                        <td><span class=\"{}\">{}</span></td>\n", difficulty_class, job.migration_difficulty));
            html.push_str(&format!("                        <td>{}</td>\n", job.migration_priority));
            html.push_str(&format!("                        <td>{}</td>\n", if job.is_critical { "⚡ Yes" } else { "" }));
            html.push_str(&format!("                        <td>{}</td>\n", job.dependency_count));
            html.push_str(&format!("                        <td>{}</td>\n", job.estimated_effort_hours));
            html.push_str("                    </tr>\n");
        }

        html.push_str("                </tbody>\n");
        html.push_str("            </table>\n");
        html.push_str("        </div>\n");

        html.push_str("    </div>\n");
        
        // Add DataTables initialization
        html.push_str("    <script>\n");
        html.push_str("        $(document).ready(function() {\n");
        html.push_str("            $('#jobsTable').DataTable({\n");
        html.push_str("                pageLength: 50,\n");
        html.push_str("                order: [[3, 'desc']],\n");
        html.push_str("                lengthMenu: [[25, 50, 100, 500, -1], [25, 50, 100, 500, 'All']],\n");
        html.push_str("                language: {\n");
        html.push_str("                    search: 'Search jobs:',\n");
        html.push_str("                    lengthMenu: 'Show _MENU_ jobs per page',\n");
        html.push_str("                    info: 'Showing _START_ to _END_ of _TOTAL_ jobs',\n");
        html.push_str("                    infoFiltered: '(filtered from _MAX_ total jobs)'\n");
        html.push_str("                }\n");
        html.push_str("            });\n");
        html.push_str("        });\n");
        html.push_str("    </script>\n");
        
        html.push_str("</body>\n</html>");

        Ok(html)
    }

    fn get_css() -> &'static str {
        r#"
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            color: #333;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }
        .container {
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            padding: 40px;
            border-radius: 12px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.2);
        }
        h1 {
            color: #2c3e50;
            font-size: 2.5em;
            margin-bottom: 30px;
            text-align: center;
        }
        h2 {
            color: #34495e;
            margin-top: 40px;
            margin-bottom: 20px;
            font-size: 1.8em;
            border-bottom: 2px solid #3498db;
            padding-bottom: 10px;
        }
        
        /* Statistics Grid */
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin: 30px 0;
        }
        .stat-card {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            border-radius: 10px;
            text-align: center;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
            transition: transform 0.2s;
        }
        .stat-card:hover {
            transform: translateY(-5px);
        }
        .stat-value {
            font-size: 3em;
            font-weight: bold;
            margin-bottom: 10px;
        }
        .stat-label {
            font-size: 1em;
            opacity: 0.9;
        }
        
        /* Difficulty Distribution */
        .summary {
            background: #f8f9fa;
            padding: 30px;
            border-radius: 10px;
            margin: 30px 0;
        }
        .difficulty-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 15px;
            margin-top: 20px;
        }
        .difficulty-card {
            padding: 25px;
            border-radius: 8px;
            text-align: center;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        .easy-bg {
            background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
            color: white;
        }
        .medium-bg {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            color: white;
        }
        .hard-bg {
            background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
            color: white;
        }
        .critical-bg {
            background: linear-gradient(135deg, #ff0844 0%, #ffb199 100%);
            color: white;
        }
        .difficulty-count {
            font-size: 2.5em;
            font-weight: bold;
            margin-bottom: 10px;
        }
        .difficulty-label {
            font-size: 1em;
            opacity: 0.95;
        }
        
        /* Wave Cards */
        .waves {
            margin: 40px 0;
        }
        .wave-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-top: 20px;
        }
        .wave-card {
            background: white;
            border: 2px solid #3498db;
            border-radius: 8px;
            padding: 20px;
            text-align: center;
            transition: all 0.3s;
        }
        .wave-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 8px 16px rgba(52, 152, 219, 0.3);
        }
        .wave-header {
            font-size: 1.5em;
            font-weight: bold;
            color: #3498db;
            margin-bottom: 10px;
        }
        .wave-count {
            font-size: 2em;
            font-weight: bold;
            color: #2c3e50;
            margin: 10px 0;
        }
        .wave-complexity {
            color: #7f8c8d;
            margin: 5px 0;
        }
        .wave-reason {
            font-size: 0.9em;
            color: #95a5a6;
            margin-top: 10px;
            font-style: italic;
        }
        
        /* Table Styles */
        .jobs {
            margin: 40px 0;
        }
        .table-info {
            color: #7f8c8d;
            margin-bottom: 15px;
            font-style: italic;
        }
        table.display {
            width: 100% !important;
        }
        .easy {
            color: #27ae60;
            font-weight: 600;
            padding: 4px 8px;
            background: #d5f4e6;
            border-radius: 4px;
        }
        .medium {
            color: #f39c12;
            font-weight: 600;
            padding: 4px 8px;
            background: #fef5e7;
            border-radius: 4px;
        }
        .hard {
            color: #e74c3c;
            font-weight: 600;
            padding: 4px 8px;
            background: #fadbd8;
            border-radius: 4px;
        }
        .wave-badge {
            background: #3498db;
            color: white;
            padding: 4px 12px;
            border-radius: 12px;
            font-size: 0.9em;
            font-weight: 600;
        }
        
        /* DataTables Customization */
        .dataTables_wrapper .dataTables_length,
        .dataTables_wrapper .dataTables_filter,
        .dataTables_wrapper .dataTables_info,
        .dataTables_wrapper .dataTables_paginate {
            margin: 15px 0;
        }
        .dataTables_wrapper .dataTables_filter input {
            padding: 8px 12px;
            border: 2px solid #ddd;
            border-radius: 6px;
            font-size: 14px;
        }
        .dataTables_wrapper .dataTables_filter input:focus {
            outline: none;
            border-color: #3498db;
        }
        "#
    }
}

impl Default for HtmlGenerator {
    fn default() -> Self {
        Self::new()
    }
}
