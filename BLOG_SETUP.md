<!-- @format -->

# Blog Setup Guide for Supabase

This guide will help you set up your blog with Supabase integration.

## 1. Create the Blog Posts Table

Run the migration script in your Supabase SQL editor:

```sql
-- Execute the contents of migrations/003_blog_posts.sql
-- This will create the blog_posts table with sample data
```

Or manually create the table:

```sql
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
```

## 2. Set Up Row Level Security (RLS)

Enable RLS and create policies:

```sql
-- Set Row Level Security (RLS) policies for public read access
ALTER TABLE blog_posts ENABLE ROW LEVEL SECURITY;

-- Allow public read access to published blog posts
CREATE POLICY "Public blog posts are viewable by everyone" ON blog_posts
    FOR SELECT USING (is_published = true);

-- Only authenticated users can modify blog posts (for admin panel)
CREATE POLICY "Authenticated users can modify blog posts" ON blog_posts
    FOR ALL USING (auth.role() = 'authenticated');
```

## 3. Environment Variables

Make sure your `.env` file contains the Supabase configuration:

```env
APP_PUBLIC_SUPABASE_URL=your_supabase_project_url
APP_PUBLIC_SUPABASE_ANON_KEY=your_supabase_anon_key
```

## 4. Test Your Setup

1. Insert a test blog post via Supabase dashboard:

```sql
INSERT INTO blog_posts (
    title,
    slug,
    content,
    excerpt,
    tags,
    is_published
) VALUES (
    'My First Blog Post',
    'my-first-blog-post',
    'This is the content of my first blog post. It can be as long as you want and supports markdown-style formatting.',
    'A brief description of what this post is about...',
    '["Test", "Blog", "First Post"]'::jsonb,
    true
);
```

2. Navigate to `http://localhost:8080/blog` to see your posts

## 5. Features Included

### ✅ What's Working Now:

- **Modern Blog Design**: Beautiful card-based layout with hover effects
- **Responsive Grid**: Adapts to different screen sizes
- **Fallback to Mock Data**: If Supabase connection fails, shows sample posts
- **Loading States**: Spinner animation while fetching data
- **Error Handling**: Displays errors gracefully
- **SEO-Friendly**: Clean URLs with slugs
- **Tag System**: Categorize posts with tags
- **Author Attribution**: Author information for each post
- **Read Time Estimation**: Approximate reading time calculation
- **Cover Images**: Optional cover images for posts

### 🚀 Ready for Enhancement:

- **Blog Post Detail Pages**: Individual post view (route exists, needs implementation)
- **Search Functionality**: Search through blog posts
- **Pagination**: For handling many blog posts
- **Categories**: Organize posts by categories
- **Comments System**: Add commenting functionality
- **Admin Panel**: Create/edit/delete posts interface

## 6. Blog Post Structure

Each blog post supports:

```typescript
{
    id: number,
    title: string,
    slug: string,          // URL-friendly version
    content: string,       // Full post content (supports markdown)
    excerpt: string,       // Brief description
    published_at: string,  // Publication date
    updated_at: string,    // Last modified date
    tags: string[],        // Array of tags
    cover_image: string,   // Optional cover image URL
    author: string,        // Post author
    is_published: boolean  // Whether post is live
}
```

## 7. Adding New Blog Posts

You can add new blog posts either:

### Via Supabase Dashboard:

1. Go to your Supabase dashboard
2. Navigate to the Table Editor
3. Select the `blog_posts` table
4. Click "Insert row"
5. Fill in the details and set `is_published` to `true`

### Via SQL:

```sql
INSERT INTO blog_posts (
    title,
    slug,
    content,
    excerpt,
    tags,
    cover_image,
    is_published
) VALUES (
    'Your Post Title',
    'your-post-slug',
    'Your full post content here...',
    'Brief excerpt or description...',
    '["Tag1", "Tag2", "Tag3"]'::jsonb,
    '/path/to/cover/image.jpg',  -- Optional
    true
);
```

## 8. Troubleshooting

### Blog Page Shows Loading Spinner Forever:

- Check your environment variables are set correctly
- Verify your Supabase URL and anon key
- Check browser console for any CORS or network errors
- Ensure RLS policies are configured correctly

### Posts Not Showing:

- Verify `is_published` is set to `true`
- Check the RLS policy allows public read access
- Look at browser network tab for API call details

### Images Not Loading:

- Use absolute URLs for cover images
- Host images in Supabase Storage or external CDN
- Ensure image URLs are publicly accessible

## 9. Next Steps

To enhance your blog further, consider:

1. **Rich Text Editor**: Integrate a markdown editor for post creation
2. **Image Upload**: Add Supabase Storage integration for images
3. **SEO Optimization**: Add meta tags, structured data
4. **Social Sharing**: Add social media sharing buttons
5. **Analytics**: Track page views and popular posts
6. **Newsletter**: Integrate email subscription functionality

Your blog is now ready to use with beautiful modern styling and Supabase integration!
