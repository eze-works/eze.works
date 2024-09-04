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
      mod: {EzeWorks, []},
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:file_system, "~> 1.0", only: :dev},
      {:typed_struct, "~> 0.3.0"},
      {:pile, "~> 0.1"},
      {:bandit, "~> 1.0"},
      {:mdex, "~> 0.1"},
      {:telemetry, "~> 1.0"}
    ]
  end
end
