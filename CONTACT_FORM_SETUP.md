<!-- @format -->

# Contact Form Supabase Integration

This guide explains how to set up and use the contact form with Supabase database integration.

## Database Setup

### 1. Create the Contact Submissions Table

Run the following SQL in your Supabase SQL editor or via migration:

```sql
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

-- Add indexes for better performance
CREATE INDEX idx_contact_submissions_created_at ON contact_submissions(created_at);
CREATE INDEX idx_contact_submissions_status ON contact_submissions(status);
CREATE INDEX idx_contact_submissions_email ON contact_submissions(email);
```

### 2. Set Row Level Security (RLS)

For security, enable RLS and create appropriate policies:

```sql
ALTER TABLE contact_submissions ENABLE ROW LEVEL SECURITY;

-- Allow authenticated users to insert new submissions
CREATE POLICY "Anyone can submit contact forms" ON contact_submissions
    FOR INSERT TO anon, authenticated
    WITH CHECK (true);

-- Only authenticated admin users can read submissions
CREATE POLICY "Only admins can read submissions" ON contact_submissions
    FOR SELECT TO authenticated
    USING (auth.jwt() ->> 'role' = 'admin');
```

## Environment Configuration

### 1. Copy Environment Template

```bash
cp .env.example .env
```

### 2. Configure Environment Variables

Update `.env` with your actual Supabase credentials:

```env
# Database Configuration (for server-side operations)
DATABASE_URL=postgresql://postgres:your_password@db.your_project.supabase.co:5432/postgres

# Supabase Configuration (for client-side operations)
APP_PUBLIC_ID=your_app_id
APP_PUBLIC_SUPABASE_URL=https://your-project.supabase.co
APP_PUBLIC_SUPABASE_ANON_KEY=your_anon_key_here
```

You can find these values in your Supabase project dashboard:

- Go to Settings → API
- Copy the Project URL for `APP_PUBLIC_SUPABASE_URL`
- Copy the `anon public` key for `APP_PUBLIC_SUPABASE_ANON_KEY`
- For DATABASE_URL, go to Settings → Database and copy the connection string

## How It Works

### Client-Side (WASM)

When running in the browser, the contact form:

1. Collects form data (name, email, message)
2. Adds metadata (timestamp, source)
3. Optionally captures user agent for analytics
4. Sends data directly to Supabase REST API using fetch

### Server-Side (Native)

When running as a server application, the contact form:

1. Uses direct PostgreSQL connection via SQLx
2. Can capture additional server-side information (IP addresses, etc.)
3. Provides better error handling and validation

### Data Structure

Each contact submission includes:

- **id**: Auto-generated UUID
- **name**: Sender's name (required)
- **email**: Sender's email (required)
- **message**: Message content (required)
- **created_at**: Timestamp (auto-generated)
- **ip_address**: Client IP (server-side only for privacy)
- **user_agent**: Browser information (optional)
- **subject**: Form subject (defaults to "Website Contact Form")
- **status**: Tracking status (defaults to "unread")
- **metadata**: Additional structured data (JSON)

## Usage

The contact form is automatically available at `/contact` route. Users can:

1. Fill out the form with their name, email, and message
2. Submit the form
3. Receive confirmation or error messages
4. Form clears automatically on successful submission

## Security Considerations

1. **Row Level Security**: Enabled on the database table
2. **API Keys**: Only the anon key is exposed (safe for client-side use)
3. **Rate Limiting**: Consider implementing rate limiting in Supabase
4. **Validation**: Both client and server-side validation
5. **IP Privacy**: IP addresses only captured server-side when necessary

## Monitoring and Management

### Viewing Submissions

You can view contact submissions in Supabase:

1. Go to Table Editor
2. Select `contact_submissions` table
3. View all submissions with filtering and sorting

### Common Queries

```sql
-- Get all unread submissions
SELECT * FROM contact_submissions WHERE status = 'unread' ORDER BY created_at DESC;

-- Mark submission as read
UPDATE contact_submissions SET status = 'read' WHERE id = 'your-uuid-here';

-- Get submissions from last 7 days
SELECT * FROM contact_submissions
WHERE created_at >= NOW() - INTERVAL '7 days'
ORDER BY created_at DESC;
```

## Troubleshooting

### Common Issues

1. **Environment Variables Not Loading**

   - Ensure `.env` file is in the `web/` directory
   - Check variable names match exactly
   - Restart development server after changes

2. **Database Connection Fails**

   - Verify DATABASE_URL is correct
   - Check Supabase project is running
   - Ensure database user has proper permissions

3. **CORS Issues**

   - Supabase automatically handles CORS for your domain
   - For local development, `localhost` should work by default

4. **RLS Policies Blocking Inserts**
   - Verify RLS policies allow anonymous insertions
   - Check if policies are too restrictive

### Development vs Production

- **Development**: Uses anon key for direct API access
- **Production**: Should use service role key for server-side operations
- **Environment**: Make sure to use different Supabase projects for dev/prod

## Future Enhancements

1. **Email Notifications**: Set up Supabase Edge Functions to send emails
2. **Spam Protection**: Add reCAPTCHA or similar
3. **File Attachments**: Extend to support file uploads
4. **Admin Dashboard**: Create admin interface for managing submissions
5. **Analytics**: Track submission patterns and response times
