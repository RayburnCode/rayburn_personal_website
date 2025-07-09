-- Create blog_posts table for the personal website
CREATE TABLE IF NOT EXISTS blog_posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    content TEXT NOT NULL,
    excerpt TEXT,
    published_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    tags JSONB DEFAULT '[]'::jsonb,
    cover_image TEXT,
    author VARCHAR(100) NOT NULL DEFAULT 'Dylan Rayburn',
    is_published BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index on slug for fast lookups
CREATE INDEX IF NOT EXISTS idx_blog_posts_slug ON blog_posts(slug);

-- Create index on published_at for ordering
CREATE INDEX IF NOT EXISTS idx_blog_posts_published_at ON blog_posts(published_at DESC);

-- Create index on is_published for filtering
CREATE INDEX IF NOT EXISTS idx_blog_posts_is_published ON blog_posts(is_published);

-- Create a function to automatically update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
DROP TRIGGER IF EXISTS update_blog_posts_updated_at ON blog_posts;
CREATE TRIGGER update_blog_posts_updated_at
    BEFORE UPDATE ON blog_posts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Insert some sample blog posts
INSERT INTO blog_posts (title, slug, content, excerpt, tags, cover_image, is_published) VALUES
(
    'Combining Finance and Technology',
    'combining-finance-and-technology',
    'In today''s rapidly evolving digital landscape, the intersection of finance and technology has become more crucial than ever. As someone with a background in both fields, I''ve seen firsthand how technology can revolutionize traditional financial processes and create new opportunities for innovation.

From building automated trading systems to developing personal finance calculators, the possibilities are endless when you combine financial knowledge with programming skills. In this post, I''ll share some insights from my journey bridging these two worlds and explore some exciting projects that demonstrate this powerful synergy.

## The Evolution of FinTech

The financial technology sector has exploded in recent years, with everything from mobile payment apps to robo-advisors changing how we interact with money. As a developer with a finance background, I''ve been fascinated by these developments and have started building my own tools to solve real-world financial problems.

## Building Financial Calculators

One of my first projects was creating a comprehensive compound interest calculator that goes beyond the basic formulas you find online. By incorporating real market data and allowing for variable contribution schedules, I was able to create a tool that provides much more accurate projections for long-term financial planning.

## The Power of Data

Modern financial applications are only as good as the data they process. Learning to work with financial APIs, clean and analyze market data, and build robust data pipelines has been crucial to my success in this space.

## Looking Forward

The future of finance is increasingly digital, and I''m excited to continue building tools that help people make better financial decisions through technology.',
    'Exploring the intersection of finance and technology and how programming skills can revolutionize traditional financial processes.',
    '["Finance", "Technology", "Programming", "FinTech"]'::jsonb,
    '/assets/blog-finance-tech.jpg',
    true
),
(
    'Getting Started with 3D Printing',
    'getting-started-with-3d-printing',
    '3D printing has opened up a whole new world of creativity and problem-solving for me. What started as curiosity about this emerging technology has evolved into a valuable skill that complements my software development work.

In this post, I''ll walk you through my 3D printing journey, from choosing my first printer to designing and printing custom solutions for everyday problems. Whether you''re a fellow developer looking to expand your maker skills or someone curious about getting into 3D printing, this guide will give you a solid foundation to start your own journey.

## Choosing Your First Printer

When I first decided to get into 3D printing, I was overwhelmed by the options available. After extensive research, I settled on the Prusa i3 MK3S+ for its reputation for reliability and print quality. While it''s not the cheapest option, the investment has paid off in consistent results and minimal troubleshooting.

## Learning CAD Design

To truly leverage 3D printing, you need to be able to design your own models. I started with Fusion 360, which offers a free license for personal use and has excellent tutorials. The parametric modeling approach clicked with my programming background – it''s like coding, but for physical objects.

## First Projects and Lessons Learned

My first successful prints were simple organizers for my desk and workshop. These projects taught me important lessons about:
- Layer adhesion and bed leveling
- Support structures and overhangs
- Material properties and print settings
- Post-processing techniques

## Advanced Techniques

As I''ve grown more comfortable with the basics, I''ve started experimenting with:
- Multi-material printing
- Custom tool creation for my workshop
- Replacement parts for household items
- Prototyping enclosures for electronics projects

## The Connection to Software Development

3D printing and software development share many similarities:
- Both require systematic problem-solving
- Iteration and testing are crucial
- Documentation and version control matter
- The maker mindset applies to both digital and physical creation

## Tips for Developers Getting Started

1. Start with proven designs from Thingiverse before creating your own
2. Learn the fundamentals of CAD design – it''s worth the investment
3. Don''t cheap out on your first printer if you can afford better
4. Join online communities for troubleshooting and inspiration
5. Keep detailed notes of successful print settings

3D printing has become an invaluable tool in my maker toolkit, allowing me to rapidly prototype ideas and create custom solutions. If you''re on the fence about diving in, I highly recommend taking the plunge – the learning curve is worth it!',
    'My journey into 3D printing and how it complements my technical skills, from choosing equipment to advanced techniques.',
    '["3D Printing", "CAD", "Maker", "Hardware"]'::jsonb,
    '/assets/blog-3d-printing.jpg',
    true
),
(
    'Building Financial Tools with Rust',
    'building-financial-tools-with-rust',
    'When it comes to financial applications, precision, performance, and safety are paramount. This is why I chose Rust as my primary language for building financial tools and calculators.

Rust''s memory safety guarantees, zero-cost abstractions, and excellent performance characteristics make it an ideal choice for financial software. In this post, I''ll share my experience building various financial tools with Rust, including compound interest calculators, portfolio analyzers, and risk assessment tools.

## Why Rust for Finance?

Financial calculations often involve large datasets and require absolute precision. Rust''s type system prevents many common bugs that could lead to calculation errors, while its performance rivals that of C and C++.

### Memory Safety
In financial software, memory leaks and buffer overflows aren''t just inconveniences – they can lead to serious security vulnerabilities. Rust''s ownership system eliminates these issues at compile time.

### Precision with Decimal Types
While floating-point arithmetic can introduce rounding errors in financial calculations, Rust''s ecosystem includes excellent decimal libraries like `rust_decimal` that ensure precise monetary calculations.

### Concurrency
Modern financial applications often need to process multiple data streams simultaneously. Rust''s ownership model makes it much easier to write safe concurrent code without data races.

## Building a Compound Interest Calculator

My first major Rust project was a comprehensive compound interest calculator. Here''s what made it special:

```rust
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

pub struct CompoundInterestCalculator {
    principal: Decimal,
    annual_rate: Decimal,
    compound_frequency: u32,
    contributions: Vec<Contribution>,
}

impl CompoundInterestCalculator {
    pub fn calculate_future_value(&self, years: u32) -> Decimal {
        // Implementation with precise decimal arithmetic
    }
}
```

## Portfolio Analysis Tools

Building on the calculator, I developed tools for portfolio analysis that can:
- Calculate portfolio returns and volatility
- Perform Monte Carlo simulations for retirement planning
- Analyze asset correlations
- Generate risk-adjusted performance metrics

## Web Interface with Dioxus

To make these tools accessible, I built a web interface using Dioxus (the framework powering this very website). Dioxus allows me to write the entire application in Rust, sharing code between the calculation engine and the UI.

## Lessons Learned

1. **Start with the math**: Get your financial formulas right before worrying about the interface
2. **Use appropriate numeric types**: Always use decimal types for monetary values
3. **Test extensively**: Financial software demands rigorous testing with known good values
4. **Performance matters**: Even web applications benefit from Rust''s speed when processing large datasets
5. **Documentation is crucial**: Financial calculations need clear documentation for verification

## Future Plans

I''m currently working on:
- Real-time market data integration
- Advanced options pricing models
- Machine learning-based market analysis tools
- Mobile applications using Rust and WebAssembly

Rust has proven to be an excellent choice for financial software development. The combination of safety, performance, and a growing ecosystem makes it ideal for building reliable financial tools that users can trust with their money.',
    'Why I chose Rust for building financial applications and calculators, and the advantages it offers for financial software development.',
    '["Rust", "Finance", "Programming", "Web Development", "FinTech"]'::jsonb,
    null,
    true
) ON CONFLICT (slug) DO NOTHING;

-- Set Row Level Security (RLS) policies for public read access
ALTER TABLE blog_posts ENABLE ROW LEVEL SECURITY;

-- Allow public read access to published blog posts
CREATE POLICY "Public blog posts are viewable by everyone" ON blog_posts
    FOR SELECT USING (is_published = true);

-- Only authenticated users can modify blog posts (for admin panel)
CREATE POLICY "Authenticated users can modify blog posts" ON blog_posts
    FOR ALL USING (auth.role() = 'authenticated');
