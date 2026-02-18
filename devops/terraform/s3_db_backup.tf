############################################
# S3 Bucket for DB Backups
############################################

resource "aws_s3_bucket" "db_backup" {
  bucket = var.db_backup_bucket

  lifecycle {
    prevent_destroy = true
  }

  tags = {
    Name        = "db-backup-bucket"
    Environment = var.environment
  }
}

############################################
# S3 Versioning (CRITICAL FOR BACKUPS)
############################################

resource "aws_s3_bucket_versioning" "db_backup" {
  bucket = aws_s3_bucket.db_backup.id

  versioning_configuration {
    status = "Enabled"
  }
}

############################################
# S3 Encryption (Default AES256)
############################################

resource "aws_s3_bucket_server_side_encryption_configuration" "db_backup" {
  bucket = aws_s3_bucket.db_backup.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

############################################
# Public Access Block
############################################

resource "aws_s3_bucket_public_access_block" "db_backup" {
  bucket = aws_s3_bucket.db_backup.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

############################################
# IAM Policy (Using aws_iam_policy_document)
############################################

data "aws_iam_policy_document" "db_backup" {
  statement {
    sid     = "AllowS3BackupAccess"
    effect  = "Allow"

    actions = [
      "s3:PutObject",
      "s3:GetObject",
      "s3:DeleteObject"
    ]

    resources = [
      "${aws_s3_bucket.db_backup.arn}/*"
    ]
  }

  statement {
    sid     = "AllowListBucket"
    effect  = "Allow"

    actions = [
      "s3:ListBucket"
    ]

    resources = [
      aws_s3_bucket.db_backup.arn
    ]
  }
}

resource "aws_iam_policy" "db_backup" {
  name   = var.db_backup_bucket_policy
  policy = data.aws_iam_policy_document.db_backup.json
}

############################################
# IAM Role for EC2
############################################

data "aws_iam_policy_document" "assume_role" {
  statement {
    effect = "Allow"

    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "db_backup" {
  name               = var.db_backup_bucket_role
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

############################################
# Attach Policy
############################################

resource "aws_iam_role_policy_attachment" "db_backup" {
  role       = aws_iam_role.db_backup.name
  policy_arn = aws_iam_policy.db_backup.arn
}

############################################
# Instance Profile
############################################

resource "aws_iam_instance_profile" "db_backup" {
  name = var.db_backup_bucket_iam_instance_profile
  role = aws_iam_role.db_backup.name
}
