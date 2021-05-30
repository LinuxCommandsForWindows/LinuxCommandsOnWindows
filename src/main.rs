use winfetch::model::{
    graphicscard,
    hostsystem,
    motherboard,
    os,
    processor,
    processorusage,
    screenres,
    uptime,
    winntkernel
};

fn main() {
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

    println!("{}", operating_system);
    println!("{}", host_system);
    println!("{}", windows_nt_kernel);
    println!("{}", motherboard);
    println!("{}", uptime);
    println!("{}", screen_resolutions);
    println!("{}", processor);
    println!("{}", graphics_card);
    println!("{}", processor_usage);
}
