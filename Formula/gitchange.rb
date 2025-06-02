class Formula
    desc "This is program for change of account to Git"
    homepage "https://github.com/andrescuello7/gitchange"
    url "https://github.com/andrescuello7/gitchange/archive/refs/tags/v1.0.0.tar.gz"
    sha256 "9391d6a81fb520cb5ff03bafc0e58dc60aba5338"
    license "MIT"
  
    def install
      system "cargo", "install", *std_cargo_args
    end
  end
  