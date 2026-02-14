class WebsearchMcp < Formula
  desc "MCP server providing DuckDuckGo web search"
  homepage "https://github.com/mizunomi32/websearch-mcp"
  version "0.1.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.1/websearch-mcp-darwin-arm64.tar.gz"
      sha256 "cf42e730bc9cd35be9679d1cc4597a4228ef6f9b49f272fe7f15120d71e64db5"
    else
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.1/websearch-mcp-darwin-amd64.tar.gz"
      sha256 "fe0909cba333b10c15dd2391fd0f1a30181cd967cf32c695ab16f6f20e70a06e"
    end
  end

  def install
    bin.install "websearch-mcp"
  end

  test do
    assert_predicate bin/"websearch-mcp", :executable?
  end
end
