defmodule EzeWorks.Telemetry do
  require Logger
  use TypedStruct

  typedstruct module: CanonicalLog, enforce: true do
    field(:method, String.t())
    field(:request_path, String.t())
    field(:query_params, String.t())
    field(:status, number())
    field(:asset?, boolean())
    field(:ip, :kernel.ip4_address())
    field(:duration_milli, number())
    field(:duration_micro, number())
  end

  def dispatch_event(
        [:bandit, :request, :stop],
        measurements,
        %{conn: %Plug.Conn{}} = meta,
        _config
      ) do
    is_asset_request = String.starts_with?(meta.conn.request_path, "/assets/")
    duration_milli = System.convert_time_unit(measurements.duration, :native, :millisecond)
    duration_micro = System.convert_time_unit(measurements.duration, :native, :microsecond)

    log_line = %EzeWorks.Telemetry.CanonicalLog{
      method: meta.conn.method,
      request_path: meta.conn.request_path,
      status: meta.conn.status,
      query_params: meta.conn.query_string,
      asset?: is_asset_request,
      ip: :inet.ntoa(meta.conn.remote_ip),
      duration_milli: duration_milli,
      duration_micro: duration_micro
    }

    {:ok, _} =
      Task.Supervisor.start_child(EzeWorks.Workers, fn ->
        write_canonical_log(log_line)
      end)
  end

  def dispatch_event([:bandit, :request, :stop], _measurements, _meta, _config) do
    # the request had an error and does not include the `conn` field, do nothing
  end

  defp write_canonical_log(%EzeWorks.Telemetry.CanonicalLog{} = log_line) do
    parts = [
      "ip=#{log_line.ip}",
      "method=#{log_line.method}",
      "status=#{log_line.status}",
      "path=#{log_line.request_path}",
      "qp=#{log_line.query_params}",
      "is-asset=#{log_line.asset?}",
      "took_micro=#{log_line.duration_micro}",
      "took_milli=#{log_line.duration_milli}"
    ]

    Logger.info("canonical-log-line #{Enum.join(parts, " ")}")
  end
end
