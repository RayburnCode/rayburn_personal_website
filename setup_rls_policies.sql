-- Fix RLS for contact_submissions table
-- Run this in your Supabase SQL Editor

-- 1. Check current policies and RLS status
SELECT schemaname, tablename, rowsecurity 
FROM pg_tables 
WHERE tablename = 'contact_submissions';

SELECT policyname, permissive, roles, cmd, qual, with_check 
FROM pg_policies 
WHERE tablename = 'contact_submissions';

-- 2. Enable Row Level Security on the table (if not already enabled)
ALTER TABLE contact_submissions ENABLE ROW LEVEL SECURITY;

-- 3. Drop existing policies if they exist (to recreate them)
DROP POLICY IF EXISTS "Anyone can submit contact forms" ON contact_submissions;
DROP POLICY IF EXISTS "Admins can read contact submissions" ON contact_submissions;

-- 4. Create a policy to allow anyone to insert contact submissions
-- This allows the contact form to work for anonymous users
CREATE POLICY "Anyone can submit contact forms" ON contact_submissions
    FOR INSERT 
    WITH CHECK (true);

-- 5. Create a policy to allow authenticated admin users to read submissions
CREATE POLICY "Admins can read contact submissions" ON contact_submissions
    FOR SELECT 
    USING (
        auth.role() = 'authenticated' 
        AND auth.jwt() ->> 'email' = 'dylan@rayburnlp.com'
    );

-- 6. Alternative: If you want to allow reading for any authenticated user, use this instead:
-- CREATE POLICY "Authenticated users can read contact submissions" ON contact_submissions
--     FOR SELECT 
--     USING (auth.role() = 'authenticated');

-- 7. Check if the table exists and has the right structure
SELECT * FROM contact_submissions LIMIT 5;

-- 8. Test that policies are working by checking policy details
SELECT policyname, permissive, roles, cmd, qual, with_check 
FROM pg_policies 
WHERE tablename = 'contact_submissions';
