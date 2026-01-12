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
        html.push_str("    <style>\n");
        html.push_str(Self::get_css());
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str("    <div class=\"container\">\n");
        html.push_str("        <h1>Control-M to Airflow Migration Analysis</h1>\n");

        html.push_str("        <div class=\"summary\">\n");
        html.push_str("            <h2>Executive Summary</h2>\n");
        html.push_str(&format!("            <p><strong>Analysis Date:</strong> {}</p>\n", output.summary.analysis_date));
        html.push_str(&format!("            <p><strong>Total Jobs:</strong> {}</p>\n", output.summary.total_jobs));
        html.push_str(&format!("            <p><strong>Total Folders:</strong> {}</p>\n", output.summary.total_folders));
        html.push_str(&format!("            <p><strong>Average Complexity:</strong> {:.2}</p>\n", output.summary.average_complexity_score));
        html.push_str(&format!("            <p><strong>Circular Dependencies:</strong> {}</p>\n", 
            if output.summary.has_circular_dependencies { "⚠️ Yes" } else { "✅ No" }));
        html.push_str("        </div>\n");

        html.push_str("        <div class=\"waves\">\n");
        html.push_str("            <h2>Migration Waves</h2>\n");
        for wave in &output.migration_waves {
            html.push_str(&format!("            <div class=\"wave\">\n"));
            html.push_str(&format!("                <h3>Wave {} ({} jobs)</h3>\n", wave.wave, wave.jobs.len()));
            html.push_str(&format!("                <p class=\"reason\">{}</p>\n", wave.reason));
            html.push_str("                <ul>\n");
            for job in &wave.jobs {
                html.push_str(&format!("                    <li>{}</li>\n", job));
            }
            html.push_str("                </ul>\n");
            html.push_str("            </div>\n");
        }
        html.push_str("        </div>\n");

        html.push_str("        <div class=\"jobs\">\n");
        html.push_str("            <h2>Job Details</h2>\n");
        html.push_str("            <table>\n");
        html.push_str("                <thead>\n");
        html.push_str("                    <tr>\n");
        html.push_str("                        <th>Job Name</th>\n");
        html.push_str("                        <th>Folder</th>\n");
        html.push_str("                        <th>Complexity</th>\n");
        html.push_str("                        <th>Difficulty</th>\n");
        html.push_str("                        <th>Priority</th>\n");
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
            html.push_str(&format!("                        <td>{}</td>\n", job.folder));
            html.push_str(&format!("                        <td>{}</td>\n", job.complexity_score));
            html.push_str(&format!("                        <td class=\"{}\">{}</td>\n", difficulty_class, job.migration_difficulty));
            html.push_str(&format!("                        <td>{}</td>\n", job.migration_priority));
            html.push_str(&format!("                        <td>{}</td>\n", job.metrics.dependency_count));
            html.push_str(&format!("                        <td>{}</td>\n", job.airflow_mapping.estimated_effort_hours));
            html.push_str("                    </tr>\n");
        }

        html.push_str("                </tbody>\n");
        html.push_str("            </table>\n");
        html.push_str("        </div>\n");

        html.push_str("    </div>\n");
        html.push_str("</body>\n</html>");

        Ok(html)
    }

    fn get_css() -> &'static str {
        r#"
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background: #f5f5f5;
        }
        .container {
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        h1 {
            color: #2c3e50;
            border-bottom: 3px solid #3498db;
            padding-bottom: 10px;
        }
        h2 {
            color: #34495e;
            margin-top: 30px;
        }
        .summary {
            background: #ecf0f1;
            padding: 20px;
            border-radius: 5px;
            margin: 20px 0;
        }
        .summary p {
            margin: 10px 0;
        }
        .wave {
            background: #fff;
            border-left: 4px solid #3498db;
            padding: 15px;
            margin: 15px 0;
            border-radius: 4px;
        }
        .wave h3 {
            margin-top: 0;
            color: #2980b9;
        }
        .reason {
            font-style: italic;
            color: #7f8c8d;
        }
        table {
            width: 100%;
            border-collapse: collapse;
            margin: 20px 0;
        }
        th, td {
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }
        th {
            background: #34495e;
            color: white;
            font-weight: 600;
        }
        tr:hover {
            background: #f5f5f5;
        }
        .easy {
            color: #27ae60;
            font-weight: 600;
        }
        .medium {
            color: #f39c12;
            font-weight: 600;
        }
        .hard {
            color: #e74c3c;
            font-weight: 600;
        }
        ul {
            list-style-type: none;
            padding-left: 0;
        }
        ul li:before {
            content: "→ ";
            color: #3498db;
            font-weight: bold;
        }
        "#
    }
}

impl Default for HtmlGenerator {
    fn default() -> Self {
        Self::new()
    }
}
