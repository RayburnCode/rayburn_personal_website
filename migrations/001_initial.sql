CREATE TABLE created_tasks (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL,
    -- Add other fields as needed for your tasks
    title TEXT,
    description TEXT,
    completed BOOLEAN DEFAULT false,
    user_id UUID REFERENCES auth.users(id)
);


CREATE TABLE blog_posts (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    content TEXT NOT NULL,
    excerpt TEXT,
    published_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()),
    updated_at TIMESTAMP WITH TIME ZONE,
    tags TEXT[] DEFAULT '{}',
    cover_image TEXT,
    author TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL
);

CREATE TABLE contact_submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address INET,  -- More appropriate type for IP addresses
    user_agent TEXT,
    subject TEXT,     -- Optional but useful for categorization
    status TEXT DEFAULT 'unread',  -- For tracking read/unread/archived
    metadata JSONB    -- For any additional unstructured data
);


CREATE TABLE projects (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL, -- 'web', '3d-printing', etc.
    technologies TEXT[] DEFAULT '{}',
    image_url TEXT,
    github_url TEXT,
    demo_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('utc'::text, now()) NOT NULL,
    featured BOOLEAN DEFAULT false
);


-- Enable RLS on all tables
ALTER TABLE created_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE blog_posts ENABLE ROW LEVEL SECURITY;
ALTER TABLE contact_submissions ENABLE ROW LEVEL SECURITY;
ALTER TABLE projects ENABLE ROW LEVEL SECURITY;

-- Blog posts - public read access
CREATE POLICY "Public blog posts are viewable by everyone" ON blog_posts
    FOR SELECT USING (true);

-- Blog posts - only authenticated users can insert/update
CREATE POLICY "Authenticated users can insert blog posts" ON blog_posts
    FOR INSERT WITH CHECK (auth.role() = 'authenticated');

-- Contact submissions - anyone can insert
CREATE POLICY "Anyone can submit contact forms" ON contact_submissions
    FOR INSERT WITH CHECK (true);

-- Projects - public read access
CREATE POLICY "Public projects are viewable by everyone" ON projects
    FOR SELECT USING (true);

-- Tasks - users can only access their own tasks
CREATE POLICY "Users can only see their own tasks" ON created_tasks
    FOR ALL USING (auth.uid() = user_id);


-- Create storage bucket for blog images
INSERT INTO storage.buckets (id, name, public) VALUES ('blog-images', 'blog-images', true);
INSERT INTO storage.buckets (id, name, public) VALUES ('project-images', 'project-images', true);

-- Storage policies
CREATE POLICY "Public blog images" ON storage.objects FOR SELECT USING (bucket_id = 'blog-images');
CREATE POLICY "Authenticated users can upload blog images" ON storage.objects FOR INSERT WITH CHECK (bucket_id = 'blog-images' AND auth.role() = 'authenticated');

-- Sample blog posts
INSERT INTO blog_posts (title, slug, content, excerpt, author, tags) VALUES
('Combining Finance and Technology', 'combining-finance-and-technology', 'How I bridge my financial expertise with my passion for coding...', 'Exploring the intersection of finance and technology...', 'Dylan', '{"Finance", "Tech"}'),
('Getting Started with 3D Printing', 'getting-started-with-3d-printing', 'My journey into 3D printing and how it complements my technical skills...', 'My journey into 3D printing and how it complements my technical skills...', 'Dylan', '{"3D Printing", "CAD"}');

-- Sample projects
INSERT INTO projects (title, description, category, technologies, github_url) VALUES
('Mortgage Calculator', 'A responsive mortgage calculator built with Rust, Dioxus, and Tailwind CSS', 'web', '{"Rust", "Dioxus", "Tailwind CSS"}', 'https://github.com/yourusername/mortgage-calculator'),
('Loan Pipeline Tracker', 'A web application for mortgage professionals to track loan applications', 'web', '{"React", "JavaScript", "Firebase"}', 'https://github.com/yourusername/loan-pipeline');