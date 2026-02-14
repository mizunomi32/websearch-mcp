class WebsearchMcp < Formula
  desc "MCP server providing DuckDuckGo web search"
  homepage "https://github.com/mizunomi32/websearch-mcp"
  version "0.1.9"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.9/websearch-mcp-darwin-arm64.tar.gz"
      sha256 "058a11617f738cd6855a51d3318da6832b8e1a412a849eaf877de51e1a09b210"
    else
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.9/websearch-mcp-darwin-amd64.tar.gz"
      sha256 "52319da141d644acca19d6ea7bc4e9e836cba72590913b0da545c56070a61c97"
    end
  end

  def install
    bin.install "websearch-mcp"
  end

  test do
    assert_predicate bin/"websearch-mcp", :executable?
  end
end
