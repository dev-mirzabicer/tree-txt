class TreeTxt < Formula
  desc "Interactive file selector and codebase exporter"
  homepage "https://github.com/dev-mirzabicer/tree-txt"
  url "https://github.com/dev-mirzabicer/tree-txt/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "sha256-placeholder" # This will be updated by the CI/CD pipeline
  license "MIT"
  head "https://github.com/dev-mirzabicer/tree-txt.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test basic functionality
    assert_match "tree-txt", shell_output("#{bin}/tree-txt --version")
    
    # Test help output
    help_output = shell_output("#{bin}/tree-txt --help")
    assert_match "Interactive file selector", help_output
    assert_match "Generate pretty-printed", help_output
  end
end