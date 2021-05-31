use winfetch::{
    model::{
        graphicscard,
        hostsystem,
        memory,
        motherboard,
        names,
        os,
        processor,
        processorusage,
        screenres,
        storage,
        uptime,
        winntkernel,
    },
    utils
};

fn main() {
    let names = names::Names::GetNames();

    let mut operating_system = os::OS::GetOperatingSystemVersion().unwrap();
    operating_system.GetOperatingSystemArchitecture().unwrap();

    let host_system = hostsystem::HostSystem::GetHostSystemInformation().unwrap();
    let windows_nt_kernel = winntkernel::WindowsNTKernel::GetCurrentWindowsNTKernelVersion().unwrap();
    let motherboard = motherboard::Motherboard::GetMotherboard().unwrap();
    let uptime = uptime::SystemUptime::GetSystemUptime().unwrap();
    let screen_resolutions = screenres::ScreenResolution::GetScreenResolution().unwrap();
    let processor = processor::Processor::GetProcessor().unwrap();
    let graphics_card = graphicscard::GraphicsCard::GetGraphicsCards().unwrap();
    let mut processor_usage = processorusage::ProcessorUsage::GetProcessorLoadPercentage().unwrap();
    processor_usage.GetProcessesCount().unwrap();
    let memory = memory::Memory::GetMemoryStatistics().unwrap();
    let drives = storage::Storage::GetStorageStatistics().unwrap();

    let mut lines = utils::GetWindowsASCIIArt().lines().map(|refstr| refstr.to_string()).collect::<Vec<String>>();
    lines[0].push_str(&format!("  {}", names));
    lines[1].push_str(&format!("{}0m  {}", utils::ANSI_ESCAPE_SEQUENCE, String::from("-").repeat(
        format!("{}@{}", names.UserName.clone().into_string().unwrap().to_ascii_lowercase(), names.ComputerName.clone().into_string().unwrap().to_ascii_lowercase()).len()).as_str()));
    lines[2].push_str(&format!("  OS{}0m: {}", utils::ANSI_ESCAPE_SEQUENCE, operating_system));
    lines[3].push_str(&format!("  Host System{}0m: {}", utils::ANSI_ESCAPE_SEQUENCE, host_system));
    lines[4].push_str(&format!("  Kernel{}0m: {}", utils::ANSI_ESCAPE_SEQUENCE, windows_nt_kernel));
    lines[5].push_str(&format!("  Motherboard{}0m: {}", utils::ANSI_ESCAPE_SEQUENCE, motherboard));
    lines[6].push_str(&format!("  System Uptime{}0m: {}", utils::ANSI_ESCAPE_SEQUENCE, uptime));
    lines[7].push_str(&format!("  Screen Resolutions(s){}0m: {}", utils::ANSI_ESCAPE_SEQUENCE, screen_resolutions));
    lines[8].push_str(&format!("                                     Processor{}0m: {}", utils::ANSI_ESCAPE_SEQUENCE, processor));
    lines[9].push_str(&format!("  Graphics Card(s){}0m: {}", utils::ANSI_ESCAPE_SEQUENCE, graphics_card));
    lines[10].push_str(&format!("  Memory{}0m: {}", utils::ANSI_ESCAPE_SEQUENCE, memory));

    println!();
    lines.into_iter().for_each(|string| {
        println!("{}", string)
    });
    println!();
}
