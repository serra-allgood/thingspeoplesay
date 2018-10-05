provider "aws" {
  region  = "us-west-2"
  version = "~> 1.37"
}

data "aws_ami" "ubuntu" {
  most_recent = true

  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-bionic-18.04-amd64-server-*"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }

  owners = ["099720109477"] # Canonical
}

resource "aws_instance" "api" {
  ami           = "${data.aws_ami.ubuntu.id}"
  instance_type = "t2.micro"
  key_name      = "${aws_key_pair.deployer.key_name}"

  tags {
    name = "thingspeoplesay"
  }
}

output "public_dns" {
  value = "${aws_instance.api.public_dns}"
}

resource "aws_key_pair" "deployer" {
  key_name   = "deployer_key"
  public_key = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQDrzIls6hPFjEceU60lMMow9aoi3NA4KjDbzG5oO93Yi1ZS1/5H9AcmHYahSnV+FGeGj3XA4F/xNxCBYbS9ytgMpLvKJ+p0NBLMahTi3ZCSLUdBy+q+xJAg4ILDAVePLqGJeljG+q9T5OCeqpYBzz44duohyKR/VQmupdGkrmZWFkqYBUzkKkpbAF6UYH4Pkct4YiOma4Pee9R4Wm8I41avxtb8WOL96qz+mk/LqvN8h64fB8mI+q+sw1ky1ke80KumBVZmeFUjeWieoZPDzfiPRjFIVuCR93TUD8QgsBSMDPV2tokowoOR/gnpoatJXNG3a094qgYpMewdHyHzNsccJhtPxh2EWLJNWGOtIUyp8s3UXyU0ClKSLk+PRlLgSMMe/8wubokX25/ggpZ3Gwd5AI2WuSGSCOoCW3vmWgMppaHGbWL9uVY5mPcAssli1gEj0aecLPnMp+jJEWbul2Ncjm5RbUFJH2Ig3DstYcyC5PDy6zOTUj17ketVGSzX8fSf13cPTwCZqlF0DZhNjRNIm85XFLpNeRUmPTHa8/bCc3P6RFbd/N4e1wYEh9+yJ9M/0A9IYz9ZeEVwZdPhGZ4qEM810wLIwYoNBW/vopSrUGtYvUMqZjus+jlJc8DADgskn8LRF8SANW/KHrRakTKi0g3AwqNCcbU7P7kRfu1h8Q== corwin.rebma@gmail.com"
}

resource "aws_eip" "main" {
  instance = "${aws_instance.api.id}"
}

output "ip" {
  value = "${aws_eip.main.public_ip}"
}

variable "db_password" {}

resource "aws_db_instance" "thingspeoplesay_production" {
  name                = "thingspeoplesay"
  allocated_storage   = 20
  engine              = "postgres"
  engine_version      = "10.4"
  instance_class      = "db.t2.micro"
  multi_az            = false
  password            = "${var.db_password}"
  storage_type        = "gp2"
  username            = "corwin"
  skip_final_snapshot = true
}

output "db_endpoint" {
  value = "${aws_db_instance.thingspeoplesay_production.endpoint}"
}
