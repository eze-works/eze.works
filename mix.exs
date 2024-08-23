defmodule EzeWorks.MixProject do
  use Mix.Project

  def project do
    [
      app: :eze_works,
      version: "0.1.0",
      elixir: "~> 1.17",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:pile, "~> 0.1"},
      {:bandit, "~> 1.0"}
    ]
  end
end
