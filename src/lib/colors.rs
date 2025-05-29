pub enum QRColor {
    Blue,
    Purple,
    Pink,
    Green,
    Orange,
    Brown,
    Teal,
    Navy,
    GradBlue,
    GradSeaSalt,
}

pub static SVG_GRADIENTS: &str = "
        <svg width=\"0\" height=\"0\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">
        <defs>
         <linearGradient id=\"gradblue\">
               <stop offset=\"0%\" stop-color=\"rgba(2,0,36,1)\" />
               <stop offset=\"35%\" stop-color=\"rgba(2,0,36,1)\" />
               <stop offset=\"100%\" stop-color=\"rgba(0,212,255,1)\" />
         </linearGradient>
        <linearGradient id=\"gradseasalt\">
               <stop offset=\"0%\" stop-color=\"#4b6cb7\" />
               <stop offset=\"100%\" stop-color=\"#182848\" />
         </linearGradient>
         </defs>
         </svg>\n
        ";

pub static SVG_DEFAULT: &str = "
    svg {
      fill: black;
    }
    svg > rect {
      display: none;
    }
";
