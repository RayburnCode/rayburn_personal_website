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

-- Add an index on created_at for efficient time-based queries
CREATE INDEX idx_contact_submissions_created_at ON contact_submissions(created_at);

-- Add an index on status for filtering
CREATE INDEX idx_contact_submissions_status ON contact_submissions(status);

-- Add an index on email for potential lookups
CREATE INDEX idx_contact_submissions_email ON contact_submissions(email);
