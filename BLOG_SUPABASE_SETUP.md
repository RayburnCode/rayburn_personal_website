<!-- @format -->

# Blog Supabase Setup Guide

## Current Issue

The blog page is not showing data from Supabase because the environment variables are set to placeholder values.

## Quick Fix - Check Browser Console

1. Open your browser's Developer Tools (F12)
2. Go to the Console tab
3. Navigate to your blog page
4. Look for log messages that show:
   - "Supabase URL: https://your-project.supabase.com"
   - "Supabase Key (first 10 chars): your_anon_k"
   - "Making request to: [URL]"
   - "Response status: [number]"

## Setup Instructions

### 1. Get Your Supabase Credentials

1. Go to your [Supabase Dashboard](https://supabase.com/dashboard)
2. Select your project (or create one if needed)
3. Go to Settings → API
4. Copy:
   - **Project URL** (something like `https://abcdefghijklmnop.supabase.co`)
   - **anon/public key** (starts with `eyJ...`)

### 2. Set Environment Variables for Development

#### Option A: Create a .env file (for development)

Create `/Users/DylanRayburn/Documents/GitHub/rayburn_personal_website/web/.env`:

```bash
APP_PUBLIC_ID=your_app_id
APP_PUBLIC_SUPABASE_URL=https://your-project-id.supabase.co
APP_PUBLIC_SUPABASE_ANON_KEY=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

#### Option B: Set environment variables before running dx serve

```bash
export APP_PUBLIC_SUPABASE_URL="https://your-project-id.supabase.co"
export APP_PUBLIC_SUPABASE_ANON_KEY="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
dx serve
```

### 3. Create the Blog Posts Table

Run this SQL in your Supabase SQL Editor:

```sql
-- Create the blog_posts table
CREATE TABLE IF NOT EXISTS blog_posts (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    content TEXT NOT NULL,
    excerpt TEXT NOT NULL,
    published_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE,
    tags JSONB DEFAULT '[]',
    cover_image TEXT,
    author TEXT NOT NULL DEFAULT 'Dylan Rayburn',
    is_published BOOLEAN DEFAULT false
);

-- Enable Row Level Security
ALTER TABLE blog_posts ENABLE ROW LEVEL SECURITY;

-- Create policy to allow public read access to published posts
CREATE POLICY "Allow public read access to published posts"
ON blog_posts FOR SELECT
TO anon
USING (is_published = true);

-- Insert some sample data
INSERT INTO blog_posts (title, slug, excerpt, content, author, tags, is_published) VALUES
(
    'Getting Started with Dioxus and Supabase',
    'getting-started-dioxus-supabase',
    'Learn how to build modern web applications with Dioxus and Supabase integration.',
    'Building modern web applications requires powerful tools and seamless data management. In this post, we''ll explore how to combine Dioxus, a modern Rust framework for building user interfaces, with Supabase, a comprehensive backend-as-a-service platform.

Dioxus provides an excellent developer experience with its React-like syntax while maintaining the performance benefits of Rust. When paired with Supabase''s real-time database, authentication, and API capabilities, you can create full-stack applications with ease.

This setup allows for type-safe communication between your frontend and backend, real-time updates, and a scalable architecture that grows with your application.',
    'Dylan Rayburn',
    '["Rust", "Dioxus", "Supabase", "Web Development"]',
    true
),
(
    'Building Responsive UIs with Tailwind CSS',
    'responsive-ui-tailwind',
    'Master the art of creating beautiful, responsive interfaces using Tailwind CSS.',
    'Tailwind CSS has revolutionized how we approach styling in modern web development. Its utility-first approach allows for rapid prototyping and consistent design systems while maintaining full customization capabilities.

In this comprehensive guide, we''ll cover advanced Tailwind techniques including custom component creation, responsive design patterns, and dark mode implementation. You''ll learn how to build professional-looking interfaces that work seamlessly across all device sizes.

The combination of Tailwind''s utility classes with modern CSS Grid and Flexbox creates powerful layout possibilities that are both performant and maintainable.',
    'Dylan Rayburn',
    '["CSS", "Tailwind", "UI Design", "Responsive Design"]',
    true
);
```

### 4. Test the Connection

1. Restart your development server:

   ```bash
   cd /Users/DylanRayburn/Documents/GitHub/rayburn_personal_website/web
   dx serve
   ```

2. Open the blog page and check the browser console for:
   - Connection status logs
   - Any error messages
   - Data fetch results

### 5. Troubleshooting

#### If you see "your-project.supabase.com" in the logs:

- Environment variables are not set correctly
- Make sure you're setting them at build time, not runtime

#### If you see a 404 error:

- The `blog_posts` table doesn't exist
- Run the SQL creation script above

#### If you see a 406 error (Not Acceptable):

- **Most common cause**: The `blog_posts` table doesn't exist in your Supabase database
- **Solution**: Run the SQL script provided above to create the table
- This error means Supabase can't find the endpoint/table you're requesting
- Check that you've created the table with the exact name `blog_posts`

#### If you see a 401/403 error:

- API key is incorrect
- RLS policies are blocking access
- Make sure the policy allows anonymous access to published posts

#### If you see CORS errors:

- This shouldn't happen with Supabase as it handles CORS automatically
- Check if your Supabase URL is correct

### 6. Next Steps

Once you have real data loading:

1. Remove the debug logging from the code
2. Add more blog posts through Supabase dashboard
3. Set up proper image hosting for cover images
4. Configure your production environment variables

## Current Fallback Behavior

The blog page now shows debug posts when:

- No data is found in Supabase (empty table)
- Connection fails (wrong credentials/URL)

This helps you understand what's happening and ensures the page isn't completely empty while you're setting up the database.
